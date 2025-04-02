mod error;

use std::env::var;
use std::fs::read_dir;
use std::path::PathBuf;

use error::{Error, Result};

fn parse_windows_path_value(path_value: String) -> Result<Vec<String>> {
    let mut all_executables = Vec::new();
    for var in path_value.split(';').filter(|v| !v.is_empty()) {
        let path = PathBuf::from(var);
        if path.exists() && path.is_dir() {
            for entry in read_dir(path)? {
                let file_name_os = entry?.file_name();
                if let Ok(file_name_string) = file_name_os.into_string() {
                    if file_name_string.ends_with(".exe") {
                        // NOTE: Unwrapping here as an if check exists
                        // fix if there are issues arising
                        let exectuable_name = file_name_string.split_once(".exe").unwrap().0;
                        all_executables.push(exectuable_name.to_owned());
                    }
                }
            }
        }
    }
    Ok(all_executables)
}

fn parse_linux_path_value(path_value: String) -> Result<Vec<String>> {
    let mut all_executables = Vec::new();
    for var in path_value.split(':').filter(|v| !v.is_empty()) {
        let path = PathBuf::from(var);
        if path.exists() && path.is_dir() {
            for entry in read_dir(path)? {
                let file_name_os = entry?.file_name();
                if let Ok(file_name_string) = file_name_os.into_string() {
                    all_executables.push(file_name_string);
                }
            }
        }

    }
    Ok(all_executables)
}

fn parse_path_value(path_value: String) -> Result<Vec<String>> {
    if cfg!(target_os = "windows") {
        parse_windows_path_value(path_value)
    } else if cfg!(target_os = "linux"){
        parse_linux_path_value(path_value)
    } else {
        Err(Error::UnSupportedOs)
    }
}

fn exists(exec_name: String, path_value: String) -> bool {
    parse_path_value(path_value).map_or(false, |s| s.contains(&exec_name))
}

pub fn executable_exists(exec_name: impl AsRef<str>) -> bool {
    let key = "PATH";
    let exec_name = exec_name.as_ref().to_owned();
    match var(key) {
        Ok(path_value) => exists(exec_name, path_value),
        Err(err) => {
            println!("{}", err);
            false
        }
    }
}
