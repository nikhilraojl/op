use std::{
    path::{Path, PathBuf},
    process::Command,
};
use walkdir::{DirEntry, WalkDir};

#[derive(Debug)]
enum Error {
    //std errors
    IoError(std::io::Error),
    StdVarError(std::env::VarError),

    // crate errors
    NoProjectProvided,
}

impl core::fmt::Display for Error {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        match self {
            Self::IoError(_err) => write!(fmt, "{self:?}"),
            Self::StdVarError(_err) => write!(fmt, "{self:?}"),
            Self::NoProjectProvided => write!(fmt, "No project name provided"),
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

type Result<T> = core::result::Result<T, Error>;

fn main() -> Result<()> {
    let pattern = std::env::args()
        .nth(1)
        .ok_or_else(|| Error::NoProjectProvided)?;

    let profile_path = std::env::var("userprofile")?;
    let projs_home_dir = Path::new(&profile_path).join("Projects");

    let checked_proj_path = check_path_exits(&projs_home_dir)?;
    let ignore_dir = projs_home_dir.join("deploys");

    let all_projs = get_projs(&checked_proj_path, &ignore_dir)?;

    for proj in &all_projs {
        let matching_project = proj.path().ends_with(&pattern);
        if matching_project {
            println!("Opening project {}", proj.path().display());
            
            let _ = std::env::set_current_dir(proj.path());

            let mut nvim_process = Command::new("nvim");
            nvim_process.arg(".");
            nvim_process.status()?;
            return Ok(());
        }
    }
    println!("No matching projects found. Only below projects are available");
    println!("{all_projs:#?}");

    Ok(())
}

fn check_path_exits(proj_path: &PathBuf) -> std::io::Result<&PathBuf> {
    let checked_proj_path = proj_path.try_exists()?;
    match checked_proj_path {
        true => Ok(proj_path),
        false => {
            let e = std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!("{} cannot be found", proj_path.display()),
            );
            Err(e)
        }
    }
}

fn get_projs(path: &PathBuf, ignore_dir: &PathBuf) -> std::io::Result<Vec<DirEntry>> {
    let mut projs_vec = Vec::<DirEntry>::new();
    for entry in WalkDir::new(path).max_depth(2) {
        let entry = entry?;
        if entry.depth() == 2 && !entry.path().starts_with(ignore_dir) {
            projs_vec.push(entry);
        }
    }
    return Ok(projs_vec);
}
