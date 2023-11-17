mod error;
mod projects;

use console::{Key, Term};
use error::{Error, Result};
use projects::Projects;
use std::env::consts::OS;
use std::env::Args;
use std::{
    path::{Path, PathBuf},
    process::Command,
};
use walkdir::DirEntry;

#[derive(Default, Debug)]
struct ListArgs {
    list: bool,
    help: bool,
}

impl ListArgs {
    fn print_help(&self) {
        println!("op --list|-l : Prints all available projects to stdout");
    }
}

#[derive(Debug)]
struct OpArgs {
    proj_name: String,
    print_path: bool,
    help: bool,
}

impl OpArgs {
    fn print_help(&self) {
        println!("Try to use one of the below commands \n");
        println!("op <project_name>            : Opens project directly in neovim");
        println!("op <project_name> --print|-p : Prints project path to stdout");
    }
}

enum ArgAction {
    ListAllProjects(ListArgs),
    OpenProject(OpArgs),
}

fn main() {
    if let Err(err) = run() {
        println!("{err}");
    }
}

fn run() -> Result<()> {
    let mut args = std::env::args();

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

        match process_arg_command(&mut args, &projects)? {
            ArgAction::OpenProject(args) => op_project(&args, &projects)?,
            ArgAction::ListAllProjects(args) => list_all_projects(&args, &projects),
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

fn process_arg_command(args: &mut Args, all_projs: &Projects) -> Result<ArgAction> {
    catch_empty_project_list(&all_projs.filtered_items)?;

    // first arg is program path and hence ignored
    args.next();

    // we need to have an initial arg to process it
    let arg = args.next().ok_or_else(|| Error::NoArgProvided)?;

    if arg.starts_with("--") || arg.starts_with("-") {
        // we go the ListAllProjects route
        let mut list_args = ListArgs::default();
        list_args.list = check_valid_flag(&arg, "list")?;
        if let Some(next_arg) = args.next() {
            list_args.help = check_valid_flag(&next_arg, "help")?;
        }
        check_no_next_arg(args)?;
        return Ok(ArgAction::ListAllProjects(list_args));
    } else {
        // we go the OpenProject route
        let mut op_args = OpArgs {
            proj_name: arg.clone(),
            print_path: false,
            help: false,
        };
        if let Some(next_arg) = args.next() {
            op_args.print_path = check_valid_flag(&next_arg, "print")?;
        }
        if let Some(next_arg) = args.next() {
            op_args.help = check_valid_flag(&next_arg, "help")?;
        }
        check_no_next_arg(args)?;
        return Ok(ArgAction::OpenProject(op_args));
    }
}

fn check_valid_flag(arg: &String, flag_name: &str) -> Result<bool> {
    // infers long and short form of flag
    // flag_name = "test"
    // long: --test
    // short: -t
    let mut short = "-".to_owned();
    let mut long = "--".to_owned();
    let short_notation = flag_name.chars().nth(0).ok_or(Error::InvalidArgs)?;
    short.push(short_notation);
    long.push_str(flag_name);

    if arg == &short || arg == &long {
        return Ok(true);
    }
    return Err(Error::InvalidArgs);
}

fn check_no_next_arg(args: &mut Args) -> Result<()> {
    if args.next().is_some() {
        return Err(Error::InvalidArgs);
    }
    Ok(())
}

fn list_all_projects(args: &ListArgs, projects: &Projects) {
    if args.help {
        args.print_help();
    } else {
        println!("Available projects:\n");
        println!("{}", projects);
    }
}

fn open_project_in_nvim(project_name: &str, all_projs: &Projects) -> Result<()> {
    if let Some(proj) = matching_project(project_name, all_projs) {
        println!("Opening project {:?}", proj.file_name());

        std::env::set_current_dir(proj.path())?;

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

fn print_work_dir(project_name: &str, all_projs: &Projects) {
    if let Some(proj) = matching_project(project_name, all_projs) {
        println!("{}", proj.path().display());
    } else {
        println!("No matching projects found. Couldn't switch to project dir'");
    }
}

fn op_project(args: &OpArgs, projects: &Projects) -> Result<()> {
    if args.help {
        args.print_help();
    } else if args.print_path {
        print_work_dir(&args.proj_name, &projects);
    } else {
        open_project_in_nvim(&args.proj_name, &projects)?
    }
    Ok(())
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
        let name = project
            .file_name()
            .to_str()
            .ok_or_else(|| Error::NoProjectsFound)?;
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
