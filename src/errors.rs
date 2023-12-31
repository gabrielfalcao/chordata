#[derive(Debug)]
pub enum Error {
    IOError(std::io::Error),
    ConfusingBaseError(ConfusingBaseError),
    DataConversionError(String),
    RegexCaptureError(String),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::IOError(e) => write!(f, "IOError: {:#?}", e),
            Error::ConfusingBaseError(e) => write!(f, "ConfusingBaseError: {:#?}", e),
            Error::DataConversionError(e) => write!(f, "DataConversionError: {}", e),
            Error::RegexCaptureError(e) => write!(f, "RegexCaptureError: {}", e),
        }
    }
}

impl std::error::Error for Error {}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::IOError(e)
    }
}

impl From<ConfusingBaseError> for Error {
    fn from(e: ConfusingBaseError) -> Self {
        Error::ConfusingBaseError(e)
    }
}

#[derive(Debug)]
pub struct ConfusingBaseError {
    reason: String,
}
impl std::fmt::Display for ConfusingBaseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.reason)
    }
}

impl ConfusingBaseError {
    pub fn new(reason: String) -> ConfusingBaseError {
        ConfusingBaseError { reason }
    }
}


pub enum Result<T>{
    Ok(T),
    Err(Error)
}
