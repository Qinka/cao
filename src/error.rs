
use std::io;

#[derive(Debug)]
pub enum Error {
    // AuthError(String),
    ProviderError(String),
    // ProviderErrorFromIO(String, io::Error),
    UnimplementedProvider(String),
    IOError(io::Error),
    MissingRequiredArgument,
    InterfaceError(String),
    HttpRequestError(Box<ureq::Error>),
    // InternalError(String),
    InterfaceFilterError(String),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for Error {}

impl From<io::Error> for Error {
    fn from(error: io::Error) -> Self{
        Error::IOError(error)
    }
}

impl From<ureq::Error> for Error {
    fn from(error: ureq::Error) -> Self {
        Error::HttpRequestError(Box::new(error))
    }
}