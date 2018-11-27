use crate::Responder;
use crate::http;

/// HTTP/1.1 RequestHeader
pub struct RequestHeader<'req> {
    method: http::Method,
    path: http::Path<'req>,
    host: String,
    headers: Vec<self::RequestHeaderKind>
}

impl<'req> RequestHeader<'req> {
    /// new HTTP/1.1 RequestHeader
    pub fn new() -> Self {
        RequestHeader {
            method: http::Method::default(),
            path: http::Path::default(),
            host: "bowl".to_owned(),
            headers: Vec::new(),
        }
    }
}

impl<'req> Responder for RequestHeader<'req> {
    fn to_string(self) -> String {
        format!("{}{}",
            format!("{} {} HTTP/1.1\n\
                 Host: {}", 
                 self.method,
                 self.path,
                 self.host),
            self.headers.iter().fold(String::new(),
                |headers, header| format!("\n{}\n{}", headers, header)))
    }
}

enum RequestHeaderKind {
    CustomHeader(String, String),
}

impl std::fmt::Display for RequestHeaderKind {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            RequestHeaderKind::CustomHeader(name, value)
                => write!(f, "{}: {}", name, value),
        }
    }
}
