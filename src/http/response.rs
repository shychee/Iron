use std::collections::HashMap;
use std::fmt;

pub trait ResponseTrait: ToString + Send {
    fn status(&self) -> u16;
    fn body(&self) -> &str;
    fn headers(&self) -> &HashMap<String, String>;
    fn set_status(&mut self, status: u16);
    fn set_body(&mut self, body: String);
    fn set_header(&mut self, key: String, value: String);
    fn new(body: String) -> Self;
}

#[derive(Debug, Clone)]
pub struct Response {
    status: u16,
    body: String,
    headers: HashMap<String, String>,
}

impl ResponseTrait for Response {
    fn status(&self) -> u16 { self.status }
    fn body(&self) -> &str { &self.body }
    fn headers(&self) -> &HashMap<String, String> { &self.headers }
    fn set_status(&mut self, status: u16) { self.status = status; }
    fn set_body(&mut self, body: String) { self.body = body; }
    fn set_header(&mut self, key: String, value: String) {
        self.headers.insert(key, value);
    }

    fn new(body: String) -> Self {
        let mut response = Response {
            status: 200,
            body,
            headers: HashMap::new(),
        };
        response.set_header("Content-Type".to_string(), "text/plain".to_string());
        response
    }
}

impl Response {
    pub fn new(body: impl Into<String>) -> Self {
        let mut response = Response {
            status: 200,
            body: body.into(),
            headers: HashMap::new(),
        };
        response.set_header("Content-Type".to_string(), "text/plain".to_string());
        response
    }
}

impl fmt::Display for Response {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "HTTP/1.1 {} {}", self.status, self.status_text())?;
        for (key, value) in &self.headers {
            writeln!(f, "{}: {}", key, value)?;
        }
        write!(f, "\r\n{}", self.body)
    }
}

impl Response {
    fn status_text(&self) -> &'static str {
        match self.status {
            200 => "OK",
            404 => "Not Found",
            500 => "Internal Server Error",
            _ => "Unknown",
        }
    }
}
