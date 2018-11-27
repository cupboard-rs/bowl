use crate::Responder;

/// HTTP/1.1 Request Body
pub struct RequestBody;

impl RequestBody {
    /// new HTTP/1.1 Request Body
    pub fn new() -> Self {
        RequestBody
    }
}

impl Responder for RequestBody {
    fn to_string(self) -> String {
        "".to_string()
    }
}
