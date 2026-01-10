use std::fmt;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Io(std::io::Error),
    InvalidGif(String),
    LzwError(String),
    QuantizationError(String),
    Internal(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Io(e) => write!(f, "IO error: {}", e),
            Error::InvalidGif(s) => write!(f, "Invalid GIF: {}", s),
            Error::LzwError(s) => write!(f, "LZW error: {}", s),
            Error::QuantizationError(s) => write!(f, "Quantization error: {}", s),
            Error::Internal(s) => write!(f, "Internal error: {}", s),
        }
    }
}

impl std::error::Error for Error {}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::Io(err)
    }
}
