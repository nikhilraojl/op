mod error;
mod list_flow;
mod main_help;
mod open_flow;
mod projects;
mod select_flow;
mod utils;

use error::{Error, Result};
use list_flow::ListAction;
use main_help::MainHelpAction;
use open_flow::OpAction;
use projects::Projects;
use select_flow::render_loop;
use std::path::Path;
use utils::ActionTrait;
use utils::{
    catch_empty_project_list, check_help_flag, check_path_exits, check_valid_flag, get_profile_path,
};

#[derive(Debug, PartialEq)]
enum ArgAction {
    ListAllProjects(ListAction),
    OpenProject(OpAction),
    MainHelp(MainHelpAction),
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

    if args.len() == 1 {
        let mut projects = Projects::new(checked_proj_path, ignore_dir, true)?;
        render_loop(&mut projects)?;
    } else {
        // first arg is generally the program path and hence skipped here
        args.next();

        let projects = Projects::new(checked_proj_path, ignore_dir, false)?;
        catch_empty_project_list(&projects.filtered_items)?;

        let action_to_perform = process_arg_command(&mut args)?;
        match action_to_perform {
            ArgAction::MainHelp(action) => action.execute(&projects)?,
            ArgAction::OpenProject(action) => action.execute(&projects)?,
            ArgAction::ListAllProjects(action) => action.execute(&projects)?,
        }
    }

    Ok(())
}

fn process_arg_command<T: Iterator<Item = String>>(args: &mut T) -> Result<ArgAction> {
    // we need to have an initial arg to process it
    let arg = args.next().ok_or_else(|| Error::NoArgProvided)?;

    if check_valid_flag(&arg, "help")? {
        return Ok(ArgAction::MainHelp(MainHelpAction));
    }

    if check_valid_flag(&arg, "list")? {
        let mut list_args = ListAction::default();
        if let Some(iarg) = &args.next() {
            list_args.help = check_help_flag(iarg, args)?;
        }
        return Ok(ArgAction::ListAllProjects(list_args));
    } else {
        // we go the OpenProject route
        let mut op_args = OpAction {
            proj_name: arg,
            print_path: false,
            help: false,
        };

        let mut next_arg = args.next();

        if let Some(iarg) = &next_arg {
            if check_valid_flag(iarg, "print")? {
                op_args.print_path = true;
                next_arg = args.next();
            }
        }

        if let Some(iarg) = next_arg {
            op_args.help = check_help_flag(&iarg, args)?;
        }

        return Ok(ArgAction::OpenProject(op_args));
    }
}

#[test]
fn test_process_main_help_action() {
    // main action
    let mut args = ["--help".to_owned()].into_iter();
    let act = process_arg_command(&mut args).unwrap();
    let exp = ArgAction::MainHelp(MainHelpAction);
    assert_eq!(act, exp);
}

#[test]
fn test_process_list_action() {
    // --list
    let mut args = ["--list".to_owned()].into_iter();
    let act = process_arg_command(&mut args).unwrap();
    let exp = ArgAction::ListAllProjects(ListAction::default());
    assert_eq!(act, exp);

    // --list --help
    let mut args = ["--list".to_owned(), "--help".to_owned()].into_iter();
    let act = process_arg_command(&mut args).unwrap();
    let list_args = ListAction { help: true };
    let exp = ArgAction::ListAllProjects(list_args);
    assert_eq!(act, exp);

    // --list --help
    let mut args = ["--list".to_owned(), "--help".to_owned(), "x".to_owned()].into_iter();
    match process_arg_command(&mut args) {
        Ok(_) => assert!(false),
        Err(_) => assert!(true),
    }
}

#[test]
fn test_process_open_action() {
    // lapce
    let mut args = ["lapce".to_owned()].into_iter();
    let act = process_arg_command(&mut args).unwrap();
    let op_args = OpAction {
        proj_name: "lapce".to_owned(),
        print_path: false,
        help: false,
    };
    let _exp = ArgAction::OpenProject(op_args);
    assert_eq!(act, _exp);

    // lapce --help
    let mut args = ["lapce".to_owned(), "--help".to_owned()].into_iter();
    let act = process_arg_command(&mut args).unwrap();
    let op_args = OpAction {
        proj_name: "lapce".to_owned(),
        print_path: false,
        help: true,
    };
    let exp = ArgAction::OpenProject(op_args);
    assert_eq!(act, exp);

    // lapce --help x
    let mut args = ["lapce".to_owned(), "--help".to_owned(), "x".to_owned()].into_iter();
    match process_arg_command(&mut args) {
        Ok(_) => assert!(false),
        Err(_) => assert!(true),
    }
}

#[test]
fn test_process_open_action_print() {
    // lapce --print
    let mut args = ["lapce".to_owned(), "--print".to_owned()].into_iter();
    let act = process_arg_command(&mut args).unwrap();
    let op_args = OpAction {
        proj_name: "lapce".to_owned(),
        print_path: true,
        help: false,
    };
    let exp = ArgAction::OpenProject(op_args);
    assert_eq!(act, exp);

    // lapce --print --help
    let mut args = [
        "lapce".to_owned(),
        "--print".to_owned(),
        "--help".to_owned(),
    ]
    .into_iter();
    let act = process_arg_command(&mut args).unwrap();
    let op_args = OpAction {
        proj_name: "lapce".to_owned(),
        print_path: true,
        help: true,
    };
    let exp = ArgAction::OpenProject(op_args);
    assert_eq!(act, exp);

    // lapce --help --print
    let mut args = [
        "lapce".to_owned(),
        "--help".to_owned(),
        "--print".to_owned(),
    ]
    .into_iter();
    match process_arg_command(&mut args) {
        Ok(_) => assert!(false),
        Err(_) => assert!(true),
    }
}
