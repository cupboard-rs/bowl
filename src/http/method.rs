/// HTTP/1.1 Method
pub enum Method {
    /// GET HTTP Response
    GET,
}

impl Default for Method {
    fn default() -> Self {
        Method::GET
    }
}

impl std::fmt::Display for Method {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Method::GET => write!(f, "GET")
        }
    }
}
