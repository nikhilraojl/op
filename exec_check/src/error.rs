#[derive(Debug)]
pub enum Error {
    IoError,
    UnSupportedOs,
}

impl From<std::io::Error> for Error {
    fn from(_value: std::io::Error) -> Self {
        Self::IoError
    }
}

pub type Result<T> = std::result::Result<T, Error>;
