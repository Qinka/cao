
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
    #[cfg(feature = "curl")]
    HttpRequestError(Box<curl::Error>),
    #[cfg(feature = "curl")]
    HttpFormError(Box<curl::FormError>),
    #[cfg(feature = "ureq")]
    HttpRequestError(Box<ureq::Error>),
    // InternalError(String),
    InterfaceFilterError(String),
    Utf8EncodingError(std::string::FromUtf8Error),
    JsonParsingError(serde_json::error::Error),
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

impl From<std::string::FromUtf8Error> for Error {
    fn from(error: std::string::FromUtf8Error) -> Self {
        Error::Utf8EncodingError(error)
    }
}


impl From<serde_json::error::Error> for Error {
    fn from(error: serde_json::error::Error) -> Self {
        Error::JsonParsingError(error)
    }
}

#[cfg(feature = "ureq")]
impl From<ureq::Error> for Error {
    fn from(error: ureq::Error) -> Self {
        Error::HttpRequestError(Box::new(error))
    }
}

#[cfg(feature = "curl")]
impl From<curl::Error> for Error {
    fn from(error: curl::Error) -> Self {
        Error::HttpRequestError(Box::new(error))
    }
}

#[cfg(feature = "curl")]
impl From<curl::FormError> for Error {
    fn from(error: curl::FormError) -> Self {
        Error::HttpFormError(Box::new(error))
    }
}

