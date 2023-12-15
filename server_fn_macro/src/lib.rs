#![cfg_attr(feature = "nightly", feature(proc_macro_span))]
#![forbid(unsafe_code)]
#![deny(missing_docs)]

//! Implementation of the `server_fn` macro.
//!
//! This crate contains the implementation of the `server_fn` macro. [`server_macro_impl`] can be used to implement custom versions of the macro for different frameworks that allow users to pass a custom context from the server to the server function.

use convert_case::{Case, Converter};
use proc_macro2::{Literal, Span, TokenStream as TokenStream2};
use proc_macro_error::abort;
use quote::{quote, quote_spanned, ToTokens};
use syn::{
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    spanned::Spanned,
    *,
};

/// The implementation of the `server_fn` macro.
/// To allow the macro to accept a custom context from the server, pass a custom server context to this function.
/// **The Context comes from the server.** Optionally, the first argument of a server function
/// can be a custom context. This context can be used to inject dependencies like the HTTP request
/// or response or other server-only dependencies, but it does *not* have access to state that exists in the client.
///
/// The paths passed into this function are used in the generated code, so they must be in scope when the macro is called.
///
/// ```ignore
/// #[proc_macro_attribute]
/// pub fn server(args: proc_macro::TokenStream, s: TokenStream) -> TokenStream {
///     match server_macro_impl(
///         args.into(),
///         s.into(),
///         Some(syn::parse_quote!(my_crate::exports::server_fn)),
///     ) {
///         Err(e) => e.to_compile_error().into(),
///         Ok(s) => s.to_token_stream().into(),
///     }
/// }
/// ```
pub fn server_macro_impl(
    args: TokenStream2,
    body: TokenStream2,
    trait_obj_wrapper: Type,
    server_fn_path: Option<Path>,
    default_path: &str,
) -> Result<TokenStream2> {
    let body = syn::parse::<ServerFnBody>(body.into())?;
    let dummy = body.to_dummy_output();
    let dummy_name = body.to_dummy_ident();
    let args = syn::parse::<ServerFnArgs>(args.into())?;

    // default values for args
    let ServerFnArgs {
        struct_name,
        prefix,
        input,
        output,
        fn_path,
    } = args;
    let prefix = prefix.unwrap_or_else(|| Literal::string(default_path));
    let fn_path = fn_path.unwrap_or_else(|| Literal::string(""));
    let input = input.unwrap_or_else(|| syn::parse_quote!(PostUrl));
    let input = codec_ident(server_fn_path.as_ref(), input);
    let output = output.unwrap_or_else(|| syn::parse_quote!(Json));
    let output = codec_ident(server_fn_path.as_ref(), output);
    // default to PascalCase version of function name if no struct name given
    let struct_name = struct_name.unwrap_or_else(|| {
        let upper_camel_case_name = Converter::new()
            .from_case(Case::Snake)
            .to_case(Case::UpperCamel)
            .convert(body.ident.to_string());
        Ident::new(&upper_camel_case_name, body.ident.span())
    });

    // build struct for type
    let mut body = body;
    let fn_name = &body.ident;
    let fn_name_as_str = body.ident.to_string();
    let vis = body.vis;
    let block = body.block;
    let attrs = body.attrs;

    let fields = body
        .inputs
        .iter_mut()
        .map(|f| {
            let typed_arg = match f {
                FnArg::Receiver(_) => {
                    return Err(syn::Error::new(
                        f.span(),
                        "cannot use receiver types in server function macro",
                    ))
                }
                FnArg::Typed(t) => t,
            };
            // allow #[server(default)] on fields — TODO is this documented?
            let mut default = false;
            let mut other_attrs = Vec::new();
            for attr in typed_arg.attrs.iter() {
                if !attr.path().is_ident("server") {
                    other_attrs.push(attr.clone());
                    continue;
                }
                attr.parse_nested_meta(|meta| {
                    if meta.path.is_ident("default") && meta.input.is_empty() {
                        default = true;
                        Ok(())
                    } else {
                        Err(meta.error(
                            "Unrecognized #[server] attribute, expected \
                             #[server(default)]",
                        ))
                    }
                })?;
            }
            typed_arg.attrs = other_attrs;
            if default {
                Ok(quote! { #[serde(default)] pub #typed_arg })
            } else {
                Ok(quote! { pub #typed_arg })
            }
        })
        .collect::<Result<Vec<_>>>()?;

    let fn_args = body.inputs.iter().filter_map(|f| match f {
        FnArg::Receiver(_) => None,
        FnArg::Typed(t) => Some(t),
    });
    let fn_args_2 = fn_args.clone();

    let field_names = body
        .inputs
        .iter()
        .filter_map(|f| match f {
            FnArg::Receiver(_) => None,
            FnArg::Typed(t) => Some(&t.pat),
        })
        .collect::<Vec<_>>();

    // check output type
    let output_arrow = body.output_arrow;
    let return_ty = body.return_ty;

    let output_ty = 'output_ty: {
        if let syn::Type::Path(pat) = &return_ty {
            if pat.path.segments[0].ident == "Result" {
                if let PathArguments::AngleBracketed(args) = &pat.path.segments[0].arguments {
                    break 'output_ty &args.args[0];
                }
            }
        }

        abort!(
            return_ty,
            "server functions should return Result<T, ServerFnError>"
        );
    };

    // build server fn path
    let server_fn_path = server_fn_path
        .map(|path| quote!(#path))
        .unwrap_or_else(|| quote! { server_fn });

    let key_env_var = match option_env!("SERVER_FN_OVERRIDE_KEY") {
        Some(_) => "SERVER_FN_OVERRIDE_KEY",
        None => "CARGO_MANIFEST_DIR",
    };

    let link_to_server_fn = format!(
        "Serialized arguments for the [`{fn_name_as_str}`] server \
         function.\n\n"
    );
    let args_docs = quote! {
        #[doc = #link_to_server_fn]
    };

    // pass through docs
    let docs = body
        .docs
        .iter()
        .map(|(doc, span)| quote_spanned!(*span=> #[doc = #doc]))
        .collect::<TokenStream2>();

    // auto-registration with inventory
    let inventory = if cfg!(feature = "ssr") {
        quote! {
            #server_fn_path::inventory::submit! {{
                use #server_fn_path::ServerFn;
                #server_fn_path::ServerFnTraitObj::new(
                    #struct_name::PATH,
                    |req| {
                        Box::pin(#struct_name::run_on_server(req))
                    }
                )
            }}
        }
    } else {
        quote! {}
    };

    // run_body in the trait implementation
    let run_body = if cfg!(feature = "ssr") {
        quote! {
            async fn run_body(self) -> Result<Self::Output, #server_fn_path::ServerFnError> {
                let #struct_name { #(#field_names),* } = self;
                #dummy_name(#(#field_names),*).await
            }
        }
    } else {
        quote! {
            async fn run_body(self) -> Result<Self::Output, #server_fn_path::ServerFnError> {
                let #struct_name { #(#field_names),* } = self;
                todo!()
            }
        }
    };

    // the actual function definition
    let func = if cfg!(feature = "ssr") {
        quote! {
            #docs
            #(#attrs)*
            #vis async fn #fn_name(#(#fn_args),*) #output_arrow #return_ty {
                #block
            }
        }
    } else {
        quote! {
            #docs
            #(#attrs)*
            #[allow(unused_variables)]
            #vis async fn #fn_name(#(#fn_args_2),*) #output_arrow #return_ty {
                todo!()
                /* #server_fn_path::call_server_fn(
                    &{
                        let prefix = #struct_name::PREFIX.to_string();
                        prefix + "/" + #struct_name::URL
                    },
                    #struct_name { #(#field_names),* },
                    #encoding
                ).await */
            }
        }
    };

    // TODO rkyv derives
    let derives = quote! {
        #server_fn_path::serde::Serialize, #server_fn_path::serde::Deserialize
    };

    // TODO reqwest
    let client = quote! {
        #server_fn_path::client::browser::BrowserClient
    };

    // TODO Actix etc
    let req = quote! {
        ::axum::http::Request<::axum::body::Body>
    };
    let res = quote! {
        ::axum::http::Response<::axum::body::Body>
    };

    // generate path
    let path = quote! {
        if #fn_path.is_empty() {
            #server_fn_path::const_format::concatcp!(
                #prefix,
                #fn_name_as_str,
                #server_fn_path::xxhash_rust::const_xxh64::xxh64(
                    concat!(env!(#key_env_var), ":", file!(), ":", line!(), ":", column!()).as_bytes(),
                    0
                )
            )
        } else {
            #server_fn_path::const_format::concatcp!(
                #prefix,
                #fn_path
            )
        }
    };

    Ok(quote::quote! {
        #args_docs
        #docs
        #[derive(Clone, Debug, #derives)]
        pub struct #struct_name {
            #(#fields),*
        }

        impl #server_fn_path::ServerFn for #struct_name {
            // TODO prefix
            const PATH: &'static str = #path;

            type Client = #client;
            type ServerRequest = #req;
            type ServerResponse = #res;
            type Output = #output_ty;
            type InputEncoding = #input;
            type OutputEncoding = #output;

            #run_body
        }

        #inventory

        #func

        #dummy
    })

    /* if let Ok(dummy) = &mut dummy {
        let ident = &mut dummy.sig.ident;
        *ident = Ident::new(&format!("__{ident}"), ident.span());
    }

    match (&mut dummy, parse_result) {
        (Ok(unexpanded), Ok(model)) => todo!(),
        (Ok(dummy), Err(_)) => todo!(),
        (Err(_), Ok(_)) => todo!(),
        (Err(_), Err(_)) => todo!(),
    } */

    /* Expands to e.g.,

    #[derive(Deserialize, Serialize)]
    struct MyServerFn {
        foo: String,
        bar: f32,
    }

    impl ServerFn for MyServerFn {
        const PATH: &'static str = "/api/my_server_fn123";

        type Client = BrowserClient;
        type ServerRequest = Request<Body>;
        type ServerResponse = Response<Body>;
        type Output = f32;
        type InputEncoding = GetUrl;
        type OutputEncoding = Json;

        fn run_body(self) -> Self::Output {
            let MyServerFn { foo, bar } = self;
            foo.len() as f32 + bar
        }
    } */
}

/// A model that is more lenient in case of a syntax error in the function body,
/// but does not actually implement the behavior of the real model. This is
/// used to improve IDEs and rust-analyzer's auto-completion behavior in case
/// of a syntax error.
struct DummyModel {
    pub attrs: Vec<Attribute>,
    pub vis: Visibility,
    pub sig: Signature,
    pub body: TokenStream2,
}

impl Parse for DummyModel {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let attrs = input.call(Attribute::parse_outer)?;
        let vis: Visibility = input.parse()?;
        let sig: Signature = input.parse()?;

        // The body is left untouched, so it will not cause an error
        // even if the syntax is invalid.
        let body: TokenStream2 = input.parse()?;

        Ok(Self {
            attrs,
            vis,
            sig,
            body,
        })
    }
}

#[derive(Debug)]
struct ServerFnArgs {
    struct_name: Option<Ident>,
    prefix: Option<Literal>,
    input: Option<Ident>,
    output: Option<Ident>,
    fn_path: Option<Literal>,
}

impl Parse for ServerFnArgs {
    fn parse(stream: ParseStream) -> syn::Result<Self> {
        // legacy 4-part arguments
        let mut struct_name: Option<Ident> = None;
        let mut prefix: Option<Literal> = None;
        let mut encoding: Option<Literal> = None;
        let mut fn_path: Option<Literal> = None;

        // new arguments: can only be keyed by name
        let mut input: Option<Ident> = None;
        let mut output: Option<Ident> = None;

        let mut use_key_and_value = false;
        let mut arg_pos = 0;

        while !stream.is_empty() {
            arg_pos += 1;
            let lookahead = stream.lookahead1();
            if lookahead.peek(Ident) {
                let key_or_value: Ident = stream.parse()?;

                let lookahead = stream.lookahead1();
                if lookahead.peek(Token![=]) {
                    stream.parse::<Token![=]>()?;
                    let key = key_or_value;
                    use_key_and_value = true;
                    if key == "name" {
                        if struct_name.is_some() {
                            return Err(syn::Error::new(
                                key.span(),
                                "keyword argument repeated: `name`",
                            ));
                        }
                        struct_name = Some(stream.parse()?);
                    } else if key == "prefix" {
                        if prefix.is_some() {
                            return Err(syn::Error::new(
                                key.span(),
                                "keyword argument repeated: `prefix`",
                            ));
                        }
                        prefix = Some(stream.parse()?);
                    } else if key == "encoding" {
                        if encoding.is_some() {
                            return Err(syn::Error::new(
                                key.span(),
                                "keyword argument repeated: `encoding`",
                            ));
                        }
                        encoding = Some(stream.parse()?);
                    } else if key == "endpoint" {
                        if fn_path.is_some() {
                            return Err(syn::Error::new(
                                key.span(),
                                "keyword argument repeated: `endpoint`",
                            ));
                        }
                        fn_path = Some(stream.parse()?);
                    } else if key == "input" {
                        if encoding.is_some() {
                            return Err(syn::Error::new(
                                key.span(),
                                "`encoding` and `input` should not both be specified",
                            ));
                        } else if input.is_some() {
                            return Err(syn::Error::new(
                                key.span(),
                                "keyword argument repeated: `input`",
                            ));
                        }
                        input = Some(stream.parse()?);
                    } else if key == "output" {
                        if encoding.is_some() {
                            return Err(syn::Error::new(
                                key.span(),
                                "`encoding` and `output` should not both be specified",
                            ));
                        } else if output.is_some() {
                            return Err(syn::Error::new(
                                key.span(),
                                "keyword argument repeated: `output`",
                            ));
                        }
                        output = Some(stream.parse()?);
                    } else {
                        return Err(lookahead.error());
                    }
                } else {
                    let value = key_or_value;
                    if use_key_and_value {
                        return Err(syn::Error::new(
                            value.span(),
                            "positional argument follows keyword argument",
                        ));
                    }
                    if arg_pos == 1 {
                        struct_name = Some(value)
                    } else {
                        return Err(syn::Error::new(value.span(), "expected string literal"));
                    }
                }
            } else if lookahead.peek(LitStr) {
                let value: Literal = stream.parse()?;
                if use_key_and_value {
                    return Err(syn::Error::new(
                        value.span(),
                        "If you use keyword arguments (e.g., `name` = Something), \
                        then you can no longer use arguments without a keyword.",
                    ));
                }
                match arg_pos {
                    1 => return Err(lookahead.error()),
                    2 => prefix = Some(value),
                    3 => encoding = Some(value),
                    4 => fn_path = Some(value),
                    _ => return Err(syn::Error::new(value.span(), "unexpected extra argument")),
                }
            } else {
                return Err(lookahead.error());
            }

            if !stream.is_empty() {
                stream.parse::<Token![,]>()?;
            }
        }

        // parse legacy encoding into input/output
        if let Some(encoding) = encoding {
            match encoding.to_string().to_lowercase().as_str() {
                "\"url\"" => {
                    input = syn::parse_quote!(PostUrl);
                    output = syn::parse_quote!(Json);
                }
                "\"cbor\"" => {
                    input = syn::parse_quote!(Cbor);
                    output = syn::parse_quote!(Cbor);
                }
                "\"getcbor\"" => {
                    input = syn::parse_quote!(GetUrl);
                    output = syn::parse_quote!(Cbor);
                }
                "\"getjson\"" => {
                    input = syn::parse_quote!(GetUrl);
                    output = syn::parse_quote!(Json);
                }
                _ => return Err(syn::Error::new(encoding.span(), "Encoding not found.")),
            }
        }

        Ok(Self {
            struct_name,
            prefix,
            input,
            output,
            fn_path,
        })
    }
}

#[derive(Debug)]
struct ServerFnBody {
    pub attrs: Vec<Attribute>,
    pub vis: syn::Visibility,
    pub async_token: Token![async],
    pub fn_token: Token![fn],
    pub ident: Ident,
    pub generics: Generics,
    pub paren_token: token::Paren,
    pub inputs: Punctuated<FnArg, Token![,]>,
    pub output_arrow: Token![->],
    pub return_ty: syn::Type,
    pub block: TokenStream2,
    pub docs: Vec<(String, Span)>,
}

impl Parse for ServerFnBody {
    fn parse(input: ParseStream) -> Result<Self> {
        let attrs: Vec<Attribute> = input.call(Attribute::parse_outer)?;
        let vis: Visibility = input.parse()?;

        let async_token = input.parse()?;

        let fn_token = input.parse()?;
        let ident = input.parse()?;
        let generics: Generics = input.parse()?;

        let content;
        let paren_token = syn::parenthesized!(content in input);

        let inputs = syn::punctuated::Punctuated::parse_terminated(&content)?;

        let output_arrow = input.parse()?;
        let return_ty = input.parse()?;

        let block = input.parse()?;

        let docs = attrs
            .iter()
            .filter_map(|attr| {
                let Meta::NameValue(attr) = &attr.meta else {
                    return None;
                };
                if !attr.path.is_ident("doc") {
                    return None;
                }

                let value = match &attr.value {
                    syn::Expr::Lit(lit) => match &lit.lit {
                        syn::Lit::Str(s) => Some(s.value()),
                        _ => return None,
                    },
                    _ => return None,
                };

                Some((value.unwrap_or_default(), attr.path.span()))
            })
            .collect();

        Ok(Self {
            vis,
            async_token,
            fn_token,
            ident,
            generics,
            paren_token,
            inputs,
            output_arrow,
            return_ty,
            block,
            attrs,
            docs,
        })
    }
}

impl ServerFnBody {
    fn to_dummy_ident(&self) -> Ident {
        Ident::new(&format!("__{}", self.ident), self.ident.span())
    }

    fn to_dummy_output(&self) -> TokenStream2 {
        let ident = self.to_dummy_ident();
        let Self {
            attrs,
            vis,
            async_token,
            fn_token,
            generics,
            inputs,
            output_arrow,
            return_ty,
            block,
            ..
        } = &self;
        quote! {
            #[doc(hidden)]
            #(#attrs)*
            #vis #async_token #fn_token #ident #generics ( #inputs ) #output_arrow #return_ty
            #block
        }
    }
}

/// Returns either the path of the codec (if it's a builtin) or the
/// original ident.
fn codec_ident(server_fn_path: Option<&Path>, ident: Ident) -> TokenStream2 {
    if let Some(server_fn_path) = server_fn_path {
        let str = ident.to_string();
        if ["GetUrl", "PostUrl", "Cbor", "Json", "Rkyv"].contains(&str.as_str()) {
            return quote! {
                #server_fn_path::codec::#ident
            };
        }
    }

    ident.into_token_stream()
}
