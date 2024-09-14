mod route;
mod group;

pub use self::route::Route;
pub use self::group::RouterGroup;
use crate::http::{HttpMethod, RequestTrait, ResponseTrait};
use regex::Regex;
use crate::middleware::Middleware;
use async_trait::async_trait;
use std::collections::HashMap;
use std::future::Future;

#[async_trait]
pub trait RouterTrait<Req: RequestTrait, Res: ResponseTrait> {
    async fn handle_request(&self, request: Req) -> Res;
    fn add_route<F, Fut>(&mut self, method: HttpMethod, pattern: &str, handler: F)
    where
        F: Fn(Req) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Res> + Send + 'static;
    fn add_middleware<F, Fut>(&mut self, middleware: F)
    where
        F: Fn(Req) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<Req, Res>> + Send + 'static;
    fn group(&mut self, prefix: &str) -> &mut RouterGroup<Req, Res>;
}

pub struct Router<Req: RequestTrait, Res: ResponseTrait> {
    routes: Vec<Route<Req, Res>>,
    groups: Vec<RouterGroup<Req, Res>>,
    middlewares: Vec<Middleware<Req, Res>>,
}

#[async_trait]
impl<Req: RequestTrait, Res: ResponseTrait> RouterTrait<Req, Res> for Router<Req, Res> {
    async fn handle_request(&self, mut request: Req) -> Res {
        // Apply global middlewares
        for middleware in &self.middlewares {
            match middleware.call(request).await {
                Ok(new_request) => request = new_request,
                Err(response) => return response,
            }
        }
    
        // Check group routes first
        for group in &self.groups {
            if request.path().starts_with(&group.prefix) {
                // Apply group middlewares
                for middleware in &group.middlewares {
                    match middleware.call(request.clone()).await {
                        Ok(new_request) => request = new_request,
                        Err(response) => return response,
                    }
                }
    
                // Check group routes
                if let Some(response) = self.match_route(&group.routes, &mut request).await {
                    return response;
                }
            }
        }
    
        // Check global routes
        if let Some(response) = self.match_route(&self.routes, &mut request).await {
            return response;
        }
    
        // Not found
        let mut response = Res::new("404 Not Found".to_string());
        response.set_status(404);
        response
    }

    fn add_route<F, Fut>(&mut self, method: HttpMethod, pattern: &str, handler: F)
    where
        F: Fn(Req) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Res> + Send + 'static
    {
        let regex_pattern = Self::create_regex_pattern(pattern);
        let regex = Regex::new(&regex_pattern).unwrap();
        let route = Route::new(method, regex, handler);
        self.routes.push(route);
    }

    fn add_middleware<F, Fut>(&mut self, middleware: F)
    where
        F: Fn(Req) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<Req, Res>> + Send + 'static
    {
        self.middlewares.push(Middleware::new(middleware));
    }

    fn group(&mut self, prefix: &str) -> &mut RouterGroup<Req, Res> {
        let group = RouterGroup::new(prefix);
        self.groups.push(group);
        self.groups.last_mut().unwrap()
    }
}

impl<Req: RequestTrait, Res: ResponseTrait> Router<Req, Res> {
    pub fn new() -> Self {
        Router {
            routes: Vec::new(),
            groups: Vec::new(),
            middlewares: Vec::new(),
        }
    }

    fn create_regex_pattern(pattern: &str) -> String {
        let regex_pattern = pattern
            .split('/')
            .map(|segment| {
                if segment.starts_with(':') {
                    format!("(?P<{}>\\w+)", &segment[1..])
                } else if segment == "*path" {
                    "(?P<path>.*)".to_string()
                } else {
                    regex::escape(segment)
                }
            })
            .collect::<Vec<String>>()
            .join("/");

        format!("^{}$", regex_pattern)
    }

    async fn match_route(&self, routes: &[Route<Req, Res>], request: &mut Req) -> Option<Res> {
        for route in routes {
            if route.method == *request.method() {
                if let Some(captures) = route.pattern.captures(request.path()) {
                    let mut params = HashMap::new();
                    for name in route.pattern.capture_names().flatten() {
                        if let Some(value) = captures.name(name) {
                            params.insert(name.to_string(), value.as_str().to_string());
                        }
                    }
                    request.set_params(params);
                    return Some((route.handler)(request.clone()).await);
                }
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::http::{Request, Response, HttpMethod};

    #[tokio::test]
    async fn test_router_add_route() {
        let mut router = Router::<Request, Response>::new();
        router.add_route(HttpMethod::GET, "/test", |_| async { Response::new("Test".to_string()) });
        assert_eq!(router.routes.len(), 1);
    }

    #[tokio::test]
    async fn test_router_handle_request() {
        let mut router = Router::<Request, Response>::new();
        router.add_route(HttpMethod::GET, "/test", |_| async { Response::new("Test".to_string()) });
        
        let request = Request::parse("GET /test HTTP/1.1\r\nHost: example.com\r\n\r\n");
        let response = router.handle_request(request).await;
        
        assert_eq!(response.status(), 200);
        assert_eq!(response.body(), "Test");
    }

    #[tokio::test]
    async fn test_router_with_params() {
        let mut router = Router::<Request, Response>::new();
        router.add_route(HttpMethod::GET, "/user/:id", |req| async move {
            Response::new(format!("User ID: {}", req.params().get("id").unwrap()))
        });

        let request = Request::parse("GET /user/123 HTTP/1.1\r\nHost: example.com\r\n\r\n");
        let response = router.handle_request(request).await;
        
        assert_eq!(response.status(), 200);
        assert_eq!(response.body(), "User ID: 123");
    }

    #[tokio::test]
    async fn test_router_with_wildcard() {
        let mut router = Router::<Request, Response>::new();
        router.add_route(HttpMethod::GET, "/files/*path", |req| async move {
            Response::new(format!("File path: {}", req.params().get("path").unwrap()))
        });

        let request = Request::parse("GET /files/documents/report.pdf HTTP/1.1\r\nHost: example.com\r\n\r\n");
        let response = router.handle_request(request).await;
        
        assert_eq!(response.status(), 200);
        assert_eq!(response.body(), "File path: documents/report.pdf");
    }

    #[tokio::test]
    async fn test_router_not_found() {
        let router = Router::<Request, Response>::new();
        let request = Request::parse("GET /nonexistent HTTP/1.1\r\nHost: example.com\r\n\r\n");
        let response = router.handle_request(request).await;
        
        assert_eq!(response.status(), 404);
        assert!(response.body().contains("Not Found"));
    }

    #[tokio::test]
    async fn test_router_with_middleware() {
        let mut router = Router::<Request, Response>::new();
        router.add_middleware(|mut req| async move {
            req.set_param("middleware".to_string(), "applied".to_string());
            Ok(req)
        });
        router.add_route(HttpMethod::GET, "/test", |req| async move {
            Response::new(format!("Middleware: {}", req.params().get("middleware").unwrap()))
        });

        let request = Request::parse("GET /test HTTP/1.1\r\nHost: example.com\r\n\r\n");
        let response = router.handle_request(request).await;
        
        assert_eq!(response.status(), 200);
        assert_eq!(response.body(), "Middleware: applied");
    }

    #[tokio::test]
    async fn test_router_group() {
        let mut router = Router::<Request, Response>::new();
        let group = router.group("/api");
        group.add_route(HttpMethod::GET, "/users", |_| async { Response::new("Users list".to_string()) });

        let request = Request::parse("GET /api/users HTTP/1.1\r\nHost: example.com\r\n\r\n");
        let response = router.handle_request(request).await;
        
        assert_eq!(response.status(), 200);
        assert_eq!(response.body(), "Users list");
    }
}