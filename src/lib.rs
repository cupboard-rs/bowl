//! bowl is responder library
#![deny(missing_docs)]
mod response;

pub use self::response::Responder;

#[cfg(test)]
mod tests {
    use crate::Responder;
    #[test]
    fn return_empty() {
        assert_eq!(().to_string(), "");
    }
}
