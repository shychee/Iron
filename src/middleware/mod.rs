use std::sync::Arc;
use crate::http::{RequestTrait, ResponseTrait};
use std::future::Future;
use std::pin::Pin;

pub type MiddlewareFunction<Req, Res> = Arc<dyn Fn(Req) -> Pin<Box<dyn Future<Output = Result<Req, Res>> + Send>> + Send + Sync>;

pub struct Middleware<Req: RequestTrait, Res: ResponseTrait> {
    pub function: MiddlewareFunction<Req, Res>,
}

impl<Req: RequestTrait, Res: ResponseTrait> Middleware<Req, Res> {
    pub fn new<F, Fut>(f: F) -> Self
    where
        F: Fn(Req) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<Req, Res>> + Send + 'static,
    {
        Middleware {
            function: Arc::new(move |req| Box::pin(f(req))),
        }
    }

    pub async fn call(&self, request: Req) -> Result<Req, Res> {
        (self.function)(request).await
    }
}