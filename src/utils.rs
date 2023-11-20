use std::env::Args;
use std::path::PathBuf;
use std::{env::consts::OS, iter::Peekable};
use walkdir::DirEntry;

use crate::error::{Error, Result};

pub fn check_valid_flag(arg: &String, flag_name: &str) -> Result<bool> {
    // infers long and short form of flag
    // flag_name = "test"
    // long: --test
    // short: -t
    let mut short = "-".to_owned();
    let mut long = "--".to_owned();
    let short_notation = flag_name.chars().nth(0).ok_or(Error::InvalidArgs)?;
    short.push(short_notation);
    long.push_str(flag_name);

    return Ok(arg == &short || arg == &long);
}

pub fn check_help_flag(arg: &String, args: &mut Args) -> Result<bool> {
    let help_flag = check_valid_flag(arg, "help")?;
    if help_flag {
        if args.next().is_some() {
            // there should be no args after --help flag
            return Err(Error::InvalidArgs);
        }
        return Ok(help_flag);
    }
    return Err(Error::InvalidArgs);
}

pub fn catch_empty_project_list(all_projs: &Vec<DirEntry>) -> Result<()> {
    if all_projs.len() == 0 {
        Err(Error::NoProjectsFound)
    } else {
        Ok(())
    }
}

pub fn check_path_exits(proj_path: PathBuf) -> std::io::Result<PathBuf> {
    let checked_proj_path = proj_path.try_exists()?;
    if checked_proj_path {
        Ok(proj_path)
    } else {
        let e = std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!("{} cannot be found", proj_path.display()),
        );
        Err(e)
    }
}

pub fn get_profile_path() -> Result<String> {
    match OS {
        "windows" => Ok(std::env::var("userprofile")?),
        "linux" => Ok(std::env::var("HOME")?),
        _ => Err(Error::UnSupportedOS),
    }
}
