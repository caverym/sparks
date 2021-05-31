use std::fmt::Formatter;

#[derive(Debug)]
pub struct Error(String);

impl Error {
    fn new<T: ToString>(error: T) -> Error {
        Error(error.to_string())
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error(e.to_string())
    }
}

impl From<Error> for std::io::Error {
    fn from(e: Error) -> Self {
        Self::new(std::io::ErrorKind::Other, e.to_string())
    }
}

impl From<discord::Error> for Error {
    fn from(e: discord::Error) -> Self {
        Self(e.to_string())
    }
}

impl From<std::string::FromUtf8Error> for Error {
    fn from(e: std::string::FromUtf8Error) -> Self {
        Self(e.to_string())
    }
}
