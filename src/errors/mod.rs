use std::fmt;
use std::fmt::Formatter;

pub mod kind;

pub struct Error {
    pub kind: kind::ErrorKind,
    pub message: String,
}

impl std::error::Error for Error {}
impl fmt::Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "kind: {}, message: {}", self.kind, self.message)
    }
}
