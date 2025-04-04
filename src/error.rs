#[derive(Debug)]
pub enum Error {
    //std errors
    Io(std::io::Error),
    StdVar(std::env::VarError),

    // crate errors
    Any(String),
    NoArgProvided,
    NoProjectsFound,
    InvalidArgs,
    UnSupportedOS,
    GitStatus,
}

impl std::fmt::Display for Error {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
        match self {
            Self::Any(err) => write!(fmt, "{err}"),
            Self::Io(err) => write!(fmt, "{err}"),
            Self::StdVar(err) => write!(fmt, "{err}"),
            Self::NoArgProvided => write!(fmt, "No argument provided"),
            Self::InvalidArgs => write!(
                fmt,
                "Invalid argument(s) provided. Try running command with --help flag"
            ),
            Self::NoProjectsFound => {
                write!(fmt, "No Project(s) found. Check README for more details")
            }
            Self::UnSupportedOS => write!(fmt, "Current OS is unsupported"),
            Self::GitStatus => write!(fmt, "Something went wrong while fetching git status"),
        }
    }
}

impl std::error::Error for Error {}
impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Self::Io(err)
    }
}

impl From<std::env::VarError> for Error {
    fn from(err: std::env::VarError) -> Self {
        Self::StdVar(err)
    }
}

pub type Result<T> = std::result::Result<T, Error>;
