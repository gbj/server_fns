use server_fn_macro_default::server;
use server_fns::ServerFnError;
use tachys::{prelude::*, tachy_reaccy::spawn::spawn_local};

#[server]
pub async fn server_fn_1(inp: i32) -> Result<i32, ServerFnError> {
    Ok(inp.wrapping_add(1))
}

#[server(encoding = "Cbor")]
pub async fn server_fn_2(inp: i32) -> Result<i32, ServerFnError> {
    Ok(inp.wrapping_add(2))
}

#[server(input = GetUrl, output = Cbor)]
pub async fn server_fn_3(inp: i32) -> Result<i32, ServerFnError> {
    Ok(inp.wrapping_add(3))
}

pub fn my_app() -> impl RenderHtml<Dom> {
    let (count, set_count) = signal(0);
    let (count2, set_count2) = signal(0);
    let (count3, set_count3) = signal(0);

    view! {
        <button
            on:click=move |_| spawn_local(async move {
                let new_count = server_fn_1(count.get()).await.expect("server fn failed");
                set_count.set(new_count);
            })
        >
            "JSON " {move || count.get()}
        </button>
        <button
            on:click=move |_| spawn_local(async move {
                let new_count = server_fn_2(count2.get()).await.expect("server fn failed");
                set_count2.set(new_count);
            })
        >
            "CBOR " {move || count2.get()}
        </button>
        <button
            on:click=move |_| spawn_local(async move {
                let new_count = server_fn_3(count2.get()).await.expect("server fn failed");
                set_count3.set(new_count);
            })
        >
            "Get/CBOR " {move || count3.get()}
        </button>
    }
}
