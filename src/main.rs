use std::{
    env, io,
    path::{Path, PathBuf},
    process,
};
use walkdir::{DirEntry, WalkDir};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let pattern = env::args().nth(1).expect("No project name provided");

    let profile_path = env::var("userprofile")?;
    let projs_home = Path::new(&profile_path).join("Projects");
    let ignore_dir = projs_home.join("deploys");
    check_path_exits(&projs_home)?;

    let all_projs = get_projs(&projs_home, &ignore_dir)?;

    for proj in &all_projs {
        let matching_project = proj.path().ends_with(&pattern);
        if matching_project {
            println!("Opening project {}", proj.path().display());
            let _ = env::set_current_dir(proj.path());
            let mut nvim_process = process::Command::new("nvim");
            nvim_process.arg(".");
            nvim_process
                .status()
                .expect("Error: Something failed, unable to start neovim");
            return Ok(());
        }
    }
    println!("No matching projects found. Only below projects are available");
    println!("{all_projs:#?}");
    Ok(())
}

fn check_path_exits(proj_path: &PathBuf) -> io::Result<()> {
    let checked_proj_path = proj_path.try_exists()?;
    if !checked_proj_path {
        let e = io::Error::new(
            io::ErrorKind::NotFound,
            format!("{} cannot be found", proj_path.display()),
        );
        return Err(e);
    }
    Ok(())
}

fn get_projs(path: &PathBuf, ignore_dir: &PathBuf) -> io::Result<Vec<DirEntry>> {
    let mut projs_vec = Vec::<DirEntry>::new();
    for entry in WalkDir::new(path).max_depth(2) {
        let entry = entry?;
        if entry.depth() == 2 && !entry.path().starts_with(ignore_dir) {
            projs_vec.push(entry);
        }
    }
    return Ok(projs_vec);
}
