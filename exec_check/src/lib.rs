use std::env::var;
use std::fs::read_dir;
use std::io;
use std::path::PathBuf;

fn exists(exec_name: String, path_value: String) -> bool {
    let all_executables = parse_path_value(path_value);
    if all_executables.contains(&exec_name) {
        return true;
    }
    false
}

fn parse_path_value(path_value: String) -> Vec<String> {
    if cfg!(target_os = "windows") {
        parse_windows_path_value(path_value).unwrap()
    } else {
        println!("Unsupported OS");
        Vec::new()
    }
}

fn parse_windows_path_value(path_value: String) -> io::Result<Vec<String>> {
    // TODO: Make this crate `Result`
    let mut all_executables = Vec::new();
    for var in path_value.split(';').filter(|v| !v.is_empty()) {
        let path = PathBuf::from(var);
        if path.exists() && path.is_dir() {
            for entry in read_dir(path)? {
                let file_name_os = entry?.file_name();
                if let Ok(file_name_string) = file_name_os.into_string() {
                    if file_name_string.ends_with(".exe") {
                        // TODO: Unwrapping here as if check exists
                        // fix if there are issues arising
                        let exectuable_name = file_name_string.split_once(".exe").unwrap().0;
                        all_executables.push(exectuable_name.to_owned());
                    }
                }
            }
            // println!("{var}");
            // DEBUG
            // break;
        } else {
            println!("NO EXIST: {var}");
        }
    }
    // println!("ALL EXECUTABLES {:?} {}", all_executables, all_executables.len());
    Ok(all_executables)
}

// TODO: Add linux path value parsing

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
