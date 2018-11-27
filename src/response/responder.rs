/// bowl responder
pub trait Responder {
    /// return a response according to the responder
    fn to_string(self) -> String;
}

impl Responder for () {
    fn to_string(self) -> String {
        "".to_owned()
    }
}
