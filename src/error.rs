#[derive(Debug)]
pub enum Error {
    //std errors
    IoError(std::io::Error),
    StdVarError(std::env::VarError),

    // crate errors
    NoArgProvided,
    NoProjectsFound,
    InvalidArgs,
    UnSupportedOS,
    FetchStatusError,
}

impl std::fmt::Display for Error {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
        match self {
            Self::IoError(_err) => write!(fmt, "{self:?}"),
            Self::StdVarError(_err) => write!(fmt, "{self:?}"),
            Self::NoArgProvided => write!(fmt, "No argument provided"),
            Self::InvalidArgs => write!(
                fmt,
                "Invalid argument(s) provided. Try running command with --help flag"
            ),
            Self::NoProjectsFound => {
                write!(fmt, "No Project(s) found. Check README for more details")
            }
            Self::UnSupportedOS => write!(fmt, "Current OS is unsupported"),
            Self::FetchStatusError => write!(fmt, "Something went wrong while fetching git status"),
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

pub type Result<T> = std::result::Result<T, Error>;
