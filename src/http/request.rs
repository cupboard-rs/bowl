mod header;
mod body;

pub use self::header::RequestHeader;
pub use self::body::RequestBody;

use crate::Responder;

/// HTTP/1.1 Request
pub struct Request<'req> {
    header: RequestHeader<'req>,
    body: RequestBody,
}

impl<'req> Request<'req> {
    /// New HTTP/1.1 Request
    pub fn new() -> Self {
        Request {
            header: RequestHeader::new(),
            body: RequestBody::new(),
        }
    }
}

impl<'req> Responder for Request<'req> {
    fn to_string(self) -> String {
        format!("{}\n\
                 \n\
                 {}",
                 self.header.to_string(),
                 self.body.to_string())
    }
}

#[test]
fn return_empty_http_request_header() {
    assert_eq!(Request::new().to_string(),
               "GET / HTTP/1.1\n\
                Host: bowl\n\n".to_owned());
}

