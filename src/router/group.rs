use super::{Route, RouterTrait};
use crate::middleware::Middleware;
use crate::http::{HttpMethod, RequestTrait, ResponseTrait};
use std::future::Future;
use async_trait::async_trait;
use regex::Regex;

pub struct RouterGroup<Req: RequestTrait, Res: ResponseTrait> {
    pub(crate) prefix: String,
    pub(crate) routes: Vec<Route<Req, Res>>,
    pub(crate) middlewares: Vec<Middleware<Req, Res>>,
}

impl<Req: RequestTrait, Res: ResponseTrait> RouterGroup<Req, Res> {
    pub fn new(prefix: &str) -> Self {
        RouterGroup {
            prefix: prefix.to_string(),
            routes: Vec::new(),
            middlewares: Vec::new(),
        }
    }
}

#[async_trait]
impl<Req: RequestTrait, Res: ResponseTrait> RouterTrait<Req, Res> for RouterGroup<Req, Res> {
    async fn handle_request(&self, _request: Req) -> Res {
        unimplemented!("RouterGroup does not handle requests directly")
    }

    fn add_route<F, Fut>(&mut self, method: HttpMethod, pattern: &str, handler: F)
    where
        F: Fn(Req) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Res> + Send + 'static,
    {
        let full_pattern = format!("{}{}", self.prefix, pattern);
        let regex_pattern = super::Router::<Req, Res>::create_regex_pattern(&full_pattern);
        let regex = Regex::new(&regex_pattern).unwrap();
        let route = Route::new(method, regex, handler);
        self.routes.push(route);
    }

    fn add_middleware<F, Fut>(&mut self, middleware: F)
    where
        F: Fn(Req) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<Req, Res>> + Send + 'static,
    {
        self.middlewares.push(Middleware::new(middleware));
    }

    fn group(&mut self, _prefix: &str) -> &mut RouterGroup<Req, Res> {
        unimplemented!("Nested groups are not supported")
    }
}