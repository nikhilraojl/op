pub mod constants;
pub mod create_projects_dir;
pub mod projects;
pub mod select_ui;

use projects::Projects;
use std::{env::consts::OS, path::PathBuf};

use crate::{
    error::{Error, Result},
    Config,
};

use self::constants::OP_CONFIG;

pub trait ActionTrait {
    fn execute(&self, config: Config) -> Result<()>;
}
pub trait HelpTrait {
    fn print_help(&self);
}

#[derive(Default)]
pub enum ShortFlag {
    #[default]
    Infer,
    Value(char),
}

pub fn check_valid_flag(arg: &String, flag_name: &str, short_name: ShortFlag) -> Result<bool> {
    // infers long and short form of flag
    // flag_name = "test"
    // long: --test
    // short: -t
    let mut short = "-".to_owned();
    let mut long = "--".to_owned();
    match short_name {
        ShortFlag::Infer => {
            let short_notation = flag_name.chars().next().ok_or(Error::InvalidArgs)?;
            short.push(short_notation);
        }
        ShortFlag::Value(arg_name) => {
            short.push(arg_name);
        }
    }
    long.push_str(flag_name);

    Ok(arg == &short || arg == &long)
}

pub fn check_help_flag<T: Iterator<Item = String>>(arg: &String, args: &mut T) -> Result<bool> {
    let help_flag = check_valid_flag(arg, "help", ShortFlag::Infer)?;
    if help_flag {
        if args.next().is_some() {
            // there should be no args after --help flag
            return Err(Error::InvalidArgs);
        }
        return Ok(help_flag);
    }
    Err(Error::InvalidArgs)
}

pub fn catch_empty_project_list(all_projs: &[PathBuf]) -> Result<()> {
    if all_projs.is_empty() {
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

pub fn validate_paths(paths: Vec<String>) -> Vec<PathBuf> {
    let mut include_paths: Vec<PathBuf> = Vec::new();
    for path in paths {
        let path = PathBuf::from(path);
        if path.exists() {
            include_paths.push(path);
        }
    }

    include_paths
}

pub fn get_projects(config: Config) -> Result<Projects> {
    let proj_dir = &config.projects_root;
    if !proj_dir.try_exists()? {
        return Err(Error::NoProjectsFound);
    }
    Projects::new(config, false)
}

pub fn get_config_path() -> Result<PathBuf> {
    let home_dir = PathBuf::from(&get_profile_path()?);
    let config_file = home_dir.join(OP_CONFIG);
    Ok(config_file)
}
