/// HTTP/1.1 Path
pub struct Path<'a>(&'a str, Option<&'a Path<'a>>);

impl<'a> Default for Path<'a> {
    fn default() -> Self {
        Path("/", None) 
    }
}

impl<'a> std::fmt::Display for Path<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Path(current, Some(next))
                => write!(f, "{}{}", current, next),
            Path(current, None) => write!(f, "{}", current),
        }
    }
}
