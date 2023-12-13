use core::fmt::Display;
use thiserror::Error;

#[derive(Debug)]
pub struct WrapError<T>(pub T);

/// This helper macro lets you call the gnarly autoref-specialization call
/// without having to worry about things like how many & you need.
/// Mostly used when you impl From<ServerFnError> for YourError
#[macro_export]
macro_rules! server_fn_error {
    () => {{
        use $crate::{ViaError, WrapError};
        (&&&&&WrapError(())).into_server_error()
    }};
    ($err:expr) => {{
        use $crate::error::{ViaError, WrapError};
        match $err {
            error => (&&&&&WrapError(error)).into_server_error(),
        }
    }};
}
/// This trait serves as the conversion method between a variety of types
/// and ServerFnError
pub trait ViaError<E> {
    fn into_server_error(&self) -> ServerFnError<E>;
}
/// This impl should catch if you feed it a ServerFnError already.
impl<E: ServerFnErrorKind + std::error::Error + Clone> ViaError<E>
    for &&&&WrapError<ServerFnError<E>>
{
    fn into_server_error(&self) -> ServerFnError<E> {
        self.0.clone()
    }
}
/// This impl should catch passing () or nothing to server_fn_error
impl ViaError<()> for &&&WrapError<()> {
    fn into_server_error(&self) -> ServerFnError<()> {
        ServerFnError::WrappedServerError(self.0.clone())
    }
}
/// This impl will catch any type that implements any type that impls
/// Error and Clone, so that it can be wrapped into ServerFnError
impl<E: std::error::Error + Clone> ViaError<E> for &&WrapError<E> {
    fn into_server_error(&self) -> ServerFnError<E> {
        ServerFnError::WrappedServerError(self.0.clone())
    }
}
/// If it doesn't impl Error, but does impl Display and Clone,
/// we can still wrap it in String form
impl<E: Display + Clone> ViaError<E> for &WrapError<E> {
    fn into_server_error(&self) -> ServerFnError<E> {
        ServerFnError::WrappedServerError(self.0.clone())
    }
}
/// This is what happens if someone tries to pass in something that does
/// not meet the above criteria
impl<E> ViaError<E> for WrapError<E> {
    fn into_server_error(&self) -> ServerFnError<E> {
        panic!("This does not Implement Error+Clone or Display+Clone. Please do that")
    }
}
/// The Error type returned by ServerFnErrors
#[derive(Debug, Error, Clone)]
pub enum ServerFnError<E = ()> {
    #[error("{0}")]
    WrappedServerError(E),
    /// Error while trying to register the server function (only occurs in case of poisoned RwLock).
    #[error("error while trying to register the server function: {0}")]
    Registration(String),
    /// Occurs on the client if there is a network error while trying to run function on server.
    #[error("error reaching server to call server function: {0}")]
    Request(String),
    /// Occurs on the server if there is an error creating an HTTP response.
    #[error("error generating HTTP response: {0}")]
    Response(String),
    /// Occurs when there is an error while actually running the function on the server.
    #[error("error running server function: {0}")]
    ServerError(String),
    /// Occurs on the client if there is an error deserializing the server's response.
    #[error("error deserializing server function results: {0}")]
    Deserialization(String),
    /// Occurs on the client if there is an error serializing the server function arguments.
    #[error("error serializing server function arguments: {0}")]
    Serialization(String),
    /// Occurs on the server if there is an error deserializing one of the arguments that's been sent.
    #[error("error deserializing server function arguments: {0}")]
    Args(String),
    /// Occurs on the server if there's a missing argument.
    #[error("missing argument {0}")]
    MissingArg(String),
}

/// We provide a conversion from a regular String to ServerFnError for you,
/// so you should be able to do this `fn function() -> Result<(), String>`
/// and handle that with `function()?`
impl From<String> for ServerFnError<String> {
    fn from(err: String) -> Self {
        server_fn_error!(err)
    }
}

//impl<E: std::error::Error + Clone> From<E> for ServerFnError<E> {
//    fn from(err: E) -> Self {
//        server_fn_error!(err)
//    }
//}
/// A type tag for ServerFnError so we can special case it
pub(crate) trait ServerFnErrorKind {}
impl ServerFnErrorKind for ServerFnError {}

 
    
    
        
        
    
    
