use regex::Regex;
use std::sync::Arc;
use crate::http::{HttpMethod, RequestTrait, ResponseTrait};
use std::future::Future;
use std::pin::Pin;

pub(crate) type RouteHandler<Req, Res> = Arc<dyn Fn(Req) -> Pin<Box<dyn Future<Output = Res> + Send>> + Send + Sync>;

pub struct Route<Req: RequestTrait, Res: ResponseTrait> {
    pub method: HttpMethod,
    pub pattern: Regex,
    pub handler: RouteHandler<Req, Res>,
}

impl<Req: RequestTrait, Res: ResponseTrait> Route<Req, Res> {
    pub fn new<F, Fut>(method: HttpMethod, pattern: Regex, handler: F) -> Self
    where
        F: Fn(Req) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Res> + Send + 'static,
    {
        Route {
            method,
            pattern,
            handler: Arc::new(move |req| Box::pin(handler(req))),
        }
    }

    pub async fn call(&self, request: Req) -> Res {
        (self.handler)(request).await
    }
}