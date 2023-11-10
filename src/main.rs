mod error;
mod projects;

use console::{Key, Term};
use error::{Error, Result};
use projects::Projects;
use std::env::consts::OS;
use std::{
    path::{Path, PathBuf},
    process::Command,
};
use walkdir::DirEntry;

enum ArgAction {
    ListProjects,
    SwitchWorkDir,
    OpenProject,
}

fn main() -> Result<()> {
    let mut args = std::env::args();
    if args.len() > 2 {
        // 2 because of args obj format
        return Err(Error::InvalidNumberOfArgs);
    };

    let profile_path = get_profile_path()?;
    let projs_home_dir = Path::new(&profile_path).join("Projects");
    let ignore_dir = projs_home_dir.join("deploys");

    let checked_proj_path = check_path_exits(projs_home_dir)?;

    let mut filter_string = String::new();
    if args.len() == 1 {
        let mut projects = Projects::new(checked_proj_path, ignore_dir, true)?;
        let term = Term::stdout();
        term.hide_cursor()?;
        println!("{projects}");
        'main: loop {
            let read_key = term.read_key().unwrap();
            if read_key == Key::ArrowUp {
                projects.select_previous();
                filter_print(&mut projects, None, &term)?;
            }
            if read_key == Key::ArrowDown {
                projects.select_next();
                filter_print(&mut projects, None, &term)?;
            }

            if let Key::Char(ch) = read_key {
                filter_string.push(ch);
                filter_print(&mut projects, Some(&filter_string), &term)?;
            }
            if read_key == Key::Backspace {
                if !filter_string.is_empty() {
                    filter_string.pop();
                }
                filter_print(&mut projects, Some(&filter_string), &term)?;
            }
            if read_key == Key::Enter {
                select_project(&mut projects)?;
                break 'main;
            }
        }
    } else {
        let projects = Projects::new(checked_proj_path, ignore_dir, false)?;
        let arg = args.nth(1).ok_or_else(|| Error::NoArgProvided)?;

        // if arg.starts_with("--") | arg.starts_with("-") {
        // let x = process_arg_command(&arg, &projects)?;
        match process_arg_command(&arg, &projects)? {
            ArgAction::OpenProject => open_project_in_nvim(&arg, &projects)?,
            ArgAction::ListProjects => {
                println!("Available projects:\n");
                println!("{}", &projects);
            }
            ArgAction::SwitchWorkDir => switch_work_dir(&arg, &projects)?,
        }
    }

    Ok(())
}

fn get_profile_path() -> Result<String> {
    match OS {
        "windows" => Ok(std::env::var("userprofile")?),
        "linux" => Ok(std::env::var("HOME")?),
        _ => Err(Error::UnSupportedOS),
    }
}

fn check_path_exits(proj_path: PathBuf) -> std::io::Result<PathBuf> {
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

fn filter_print(
    projects: &mut Projects,
    filter_string: Option<&String>,
    term: &Term,
) -> Result<()> {
    term.clear_to_end_of_screen()?;
    term.clear_last_lines(projects.filtered_items.len())?;
    if let Some(filter_string) = filter_string {
        term.clear_last_lines(1)?;
        println!("Find: {filter_string}");
        projects.filter_project_list(filter_string)?;
    }
    print_projects(projects);
    Ok(())
}

fn print_projects(projects: &mut Projects) {
    if projects.filtered_items.len() > 0 {
        println!("{projects}");
    }
}

fn process_arg_command(arg: &str, all_projs: &Projects) -> Result<ArgAction> {
    catch_empty_project_list(&all_projs.filtered_items)?;
    if arg.starts_with("--") | arg.starts_with("-") {
        match arg {
            "-l" | "--list" => Ok(ArgAction::ListProjects),
            "-w" | "--workdir" => Ok(ArgAction::SwitchWorkDir),
            _ => Err(Error::InvalidArg),
        }
    } else {
        Ok(ArgAction::OpenProject)
    }
}

fn open_project_in_nvim(project_name: &str, all_projs: &Projects) -> Result<()> {
    if let Some(proj) = matching_project(project_name, all_projs) {
        println!("Opening project {:?}", proj.file_name());

        // TODO: Handle Result here
        let _ = std::env::set_current_dir(proj.path());

        let mut nvim_process = Command::new("nvim");
        nvim_process.arg(".");
        nvim_process.status()?;
        println!("Closing project {:?}", proj.file_name());
        std::process::exit(0);
    } else {
        println!("No matching projects found. Only below projects are available");
        println!("{all_projs}");
        Ok(())
    }
}

fn switch_work_dir(project_name: &str, all_projs: &Projects) -> Result<()> {
    if let Some(proj) = matching_project(project_name, all_projs) {
        let mut pwsh = Command::new("pwsh");
        pwsh.arg("-c");
        pwsh.arg(format!("cd {:?}", proj.path()));
        pwsh.status()?;
        Ok(())
    } else {
        println!("No matching projects found. Couldn't switch to project dir'");
        Ok(())
    }
}

fn matching_project<'a>(project_name: &str, all_projs: &'a Projects) -> Option<&'a DirEntry> {
    for proj in &all_projs.filtered_items {
        let matching_project = proj.path().ends_with(project_name);
        if matching_project {
            return Some(proj);
        }
    }
    return None;
}

fn select_project(projects: &mut Projects) -> Result<()> {
    catch_empty_project_list(&projects.filtered_items)?;
    let project = projects.filtered_items.get(projects.selected);
    if let Some(project) = project {
        let name = project.file_name().to_str().unwrap();
        open_project_in_nvim(name, &projects)?;
    }
    Ok(())
}

fn catch_empty_project_list(all_projs: &Vec<DirEntry>) -> Result<()> {
    if all_projs.len() == 0 {
        Err(Error::NoProjectsFound)
    } else {
        Ok(())
    }
}
