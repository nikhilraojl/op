pub mod constants;
pub mod create_projects_dir;
pub mod projects;
pub mod select_ui;

use constants::{OP_INCLUDE, PROJECTS_DIR};
use projects::Projects;
use std::{
    env::consts::OS,
    fs::read_to_string,
    path::{Path, PathBuf},
};

use crate::error::{Error, Result};

pub trait ActionTrait {
    fn execute(&self) -> Result<()>;
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

pub fn catch_empty_project_list(all_projs: &Vec<PathBuf>) -> Result<()> {
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

pub fn get_paths_to_include(project_path: &PathBuf) -> Vec<PathBuf> {
    let op_include = Path::new(project_path).join(OP_INCLUDE);
    let mut include_paths: Vec<PathBuf> = Vec::new();
    if !op_include.exists() {
        return include_paths;
    }
    let file = read_to_string(op_include).unwrap();

    for line in file.lines() {
        let path = PathBuf::from(line);
        if path.exists() && !path.starts_with("#") {
            // entries `#` are not considered
            include_paths.push(path);
        }
    }

    return include_paths;
}

pub fn get_project_dir() -> Result<PathBuf> {
    // returns error only if profile doesn't exist
    // unchecked for `PROJECTS_DIR`
    let profile_path = get_profile_path()?;
    let proj_dir = Path::new(&profile_path).join(PROJECTS_DIR);
    return Ok(proj_dir);
}

pub fn get_projects() -> Result<Projects> {
    let proj_dir = get_project_dir()?;
    if !proj_dir.try_exists()? {
        return Err(Error::NoProjectsFound);
    }
    return Projects::new(proj_dir, false);
}
