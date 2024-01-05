use crate::projects::Projects;
use std::env::consts::OS;
use std::path::{Path, PathBuf};
use walkdir::DirEntry;

use crate::error::{Error, Result};

pub trait ActionTrait {
    fn print_help(&self);
    fn execute(&self, projects: &Projects) -> Result<()>;
}

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

pub fn check_help_flag<T: Iterator<Item = String>>(arg: &String, args: &mut T) -> Result<bool> {
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

pub fn get_project_path() -> Result<PathBuf> {
    let profile_path = get_profile_path()?;
    let proj_dir_1 = Path::new(&profile_path).join("Projects");
    let proj_dir_2 = Path::new(&profile_path).join("projects");

    if proj_dir_1.try_exists()? {
        Ok(proj_dir_1)
    } else if proj_dir_2.try_exists()? {
        Ok(proj_dir_2)
    } else {
        let e = std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!("{} cannot be found", proj_dir_2.display()),
        );
        Err(Error::IoError(e))
    }
}

fn get_profile_path() -> Result<String> {
    match OS {
        "windows" => Ok(std::env::var("userprofile")?),
        "linux" => Ok(std::env::var("HOME")?),
        _ => Err(Error::UnSupportedOS),
    }
}
