use http::Request;
use std::task::Context;
use std::task::Poll;
use tower::Layer;
use tower::Service;

#[derive(Clone, Debug)]
pub struct LoggingService<S> {
    inner: S,
}

/// Middleware to use [`RequestId`]
impl<S> LoggingService<S> {
    pub fn new(inner: S) -> Self {
        Self { inner }
    }
}

impl<B, S> Service<Request<B>> for LoggingService<S>
where
    S: Service<Request<B>>,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = S::Future;

    #[inline]
    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, mut req: Request<B>) -> Self::Future {
        println!("got a request");
        self.inner.call(req)
    }
}

/// Layer to apply [`LoggingService`] middleware.
#[derive(Clone, Debug)]
pub struct LoggingLayer;

impl<S> Layer<S> for LoggingLayer {
    type Service = LoggingService<S>;

    fn layer(&self, inner: S) -> Self::Service {
        LoggingService { inner }
    }
}
