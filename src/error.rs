#[derive(Debug)]
pub enum Error {
    //std errors
    IoError(std::io::Error),
    StdVarError(std::env::VarError),

    // crate errors
    NoArgProvided,
    NoProjectsFound,
    InvalidNumberOfArgs,
    InvalidArg,
    UnSupportedOS,
}

impl core::fmt::Display for Error {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        match self {
            Self::IoError(_err) => write!(fmt, "{self:?}"),
            Self::StdVarError(_err) => write!(fmt, "{self:?}"),
            Self::NoArgProvided => write!(fmt, "No argument provided"),
            Self::InvalidNumberOfArgs => write!(fmt, "One argument is expected"),
            Self::InvalidArg => write!(fmt, "Invalid argument"),
            Self::NoProjectsFound => write!(fmt, "No Projects found"),
            Self::UnSupportedOS => write!(fmt, "Current OS is unsupported"),
        }
    }
}

impl std::error::Error for Error {}
impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Self::IoError(err)
    }
}

impl From<std::env::VarError> for Error {
    fn from(err: std::env::VarError) -> Self {
        Self::StdVarError(err)
    }
}

pub type Result<T> = core::result::Result<T, Error>;
