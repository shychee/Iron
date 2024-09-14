use std::collections::HashMap;
use super::HttpMethod;
use async_trait::async_trait;

#[async_trait]
pub trait RequestTrait: Clone + Send + Sync {
    fn method(&self) -> &HttpMethod;
    fn path(&self) -> &str;
    fn body(&self) -> &str;
    fn params(&self) -> &HashMap<String, String>;
    fn headers(&self) -> &HashMap<String, String>;
    fn set_param(&mut self, key: String, value: String);
    fn set_params(&mut self, params: HashMap<String, String>);
    fn parse(request: &str) -> Self;
}

#[derive(Debug, Clone)]
pub struct Request {
    method: HttpMethod,
    path: String,
    body: String,
    params: HashMap<String, String>,
    headers: HashMap<String, String>,
}

#[async_trait]
impl RequestTrait for Request {
    fn method(&self) -> &HttpMethod { &self.method }
    fn path(&self) -> &str { &self.path }
    fn body(&self) -> &str { &self.body }
    fn params(&self) -> &HashMap<String, String> { &self.params }
    fn headers(&self) -> &HashMap<String, String> { &self.headers }
    fn set_param(&mut self, key: String, value: String) {
        self.params.insert(key, value);
    }
    fn set_params(&mut self, params: HashMap<String, String>) {
        self.params = params;
    }
    fn parse(request: &str) -> Self {
        let lines: Vec<&str> = request.lines().collect();
        let first_line = lines[0];
        let parts: Vec<&str> = first_line.split_whitespace().collect();
        
        let method = match parts[0] {
            "GET" => HttpMethod::GET,
            "POST" => HttpMethod::POST,
            "PUT" => HttpMethod::PUT,
            "DELETE" => HttpMethod::DELETE,
            _ => HttpMethod::GET,
        };
        
        let path = parts[1].to_string();
        let body = request.split("\r\n\r\n").nth(1).unwrap_or("").to_string();

        let mut headers = HashMap::new();
        for line in &lines[1..] {
            if line.is_empty() { break; }
            let parts: Vec<&str> = line.splitn(2, ": ").collect();
            if parts.len() == 2 {
                headers.insert(parts[0].to_string(), parts[1].to_string());
            }
        }

        Request { method, path, body, params: HashMap::new(), headers }
    }
}