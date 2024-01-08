use crate::{projects::Projects, DEPLOYS_DIR, PROJECTS_DIR};
use std::{env::consts::OS, path::Path};
use walkdir::DirEntry;

use crate::error::{Error, Result};

pub trait ActionTrait {
    fn execute(&self) -> Result<()>;
    fn get_projects() -> Result<Projects> {
        let profile_path = get_profile_path()?;
        let proj_dir = Path::new(&profile_path).join(PROJECTS_DIR);
        if !proj_dir.try_exists()? {
            return Err(Error::NoProjectsFound);
        }
        let ignore_dir = proj_dir.join(DEPLOYS_DIR);

        return Projects::new(proj_dir, ignore_dir, false);
    }
}
pub trait HelpTrait {
    fn print_help(&self);
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

pub fn get_profile_path() -> Result<String> {
    match OS {
        "windows" => Ok(std::env::var("userprofile")?),
        "linux" => Ok(std::env::var("HOME")?),
        _ => Err(Error::UnSupportedOS),
    }
}
