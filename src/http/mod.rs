mod method;
mod request;
mod response;

pub use method::HttpMethod;
pub use request::{Request, RequestTrait};
pub use response::{Response, ResponseTrait};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_request() {
        let request_str = "GET /users/123 HTTP/1.1\r\nHost: example.com\r\n\r\n";
        let request = Request::parse(request_str);
        assert_eq!(request.method(), &HttpMethod::GET);
        assert_eq!(request.path(), "/users/123");
        assert_eq!(request.headers().get("Host"), Some(&"example.com".to_string()));
    }

    #[test]
    fn test_response_to_string() {
        let mut response = Response::new("Hello, World!");
        response.set_status(200);
        response.set_header("Content-Type".to_string(), "text/plain".to_string());
        let response_str = response.to_string();
        assert!(response_str.contains("HTTP/1.1 200 OK"));
        assert!(response_str.contains("Content-Type: text/plain"));
        assert!(response_str.contains("Hello, World!"));
    }
}