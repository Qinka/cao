use std::{array::TryFromSliceError, io, num::ParseIntError};

use reqwest::header::InvalidHeaderValue;

pub type DynError = dyn std::fmt::Debug + Send + Sync + 'static;

#[derive(Debug)]
pub enum Error {
  Catch(Box<DynError>),
  Reason(String),
  // // AuthError(String),
  // ProviderError(String),
  // // ProviderErrorFromIO(String, io::Error),
  // UnimplementedProvider(String),
  // IOError(io::Error),
  // MissingRequiredArgument,
  // InterfaceError(String),
  // HttpRequestError(Box<reqwest::Error>),
  // HttpFailed(String),
  // HeaderParseError(InvalidHeaderValue),
  // // InternalError(String),
  // InterfaceFilterError(String),
  // Utf8EncodingError(std::string::FromUtf8Error),
  // JsonParsingError(serde_json::error::Error),
  // HashError(digest::InvalidLength),
}

impl Error {
  pub fn http_failed(message: String) -> Self {
    Self::Reason(message)
  }
}

impl std::fmt::Display for Error {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{:?}", self)
  }
}

impl std::error::Error for Error {}

impl From<io::Error> for Error {
  fn from(error: io::Error) -> Self {
    Error::Catch(Box::new(error))
  }
}

impl From<std::string::FromUtf8Error> for Error {
  fn from(error: std::string::FromUtf8Error) -> Self {
    Error::Catch(Box::new(error))
  }
}

impl From<serde_json::error::Error> for Error {
  fn from(error: serde_json::error::Error) -> Self {
    Error::Catch(Box::new(error))
  }
}

impl From<reqwest::Error> for Error {
  fn from(error: reqwest::Error) -> Self {
    Error::Catch(Box::new(error))
  }
}

impl From<InvalidHeaderValue> for Error {
  fn from(error: InvalidHeaderValue) -> Self {
    Error::Catch(Box::new(error))
  }
}

impl From<digest::InvalidLength> for Error {
  fn from(error: digest::InvalidLength) -> Self {
    Error::Catch(Box::new(error))
  }
}

impl From<TryFromSliceError> for Error {
  fn from(error: TryFromSliceError) -> Self {
    Error::Catch(Box::new(error))
  }
}

impl From<ParseIntError> for Error {
  fn from(error: ParseIntError) -> Self {
    Error::Catch(Box::new(error))
  }
}

pub type Result<T> = std::result::Result<T, Error>;
