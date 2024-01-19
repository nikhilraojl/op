mod create_layout;
mod create_projects_dir;
mod error;
mod include_flow;
mod list_flow;
mod main_help_flow;
mod open_flow;
mod projects;
mod select_flow;
mod utils;

use std::path::{Path, PathBuf};

use create_layout::CreateLayout;
use create_projects_dir::create_projects_dir;
use error::{Error, Result};
use include_flow::IncludeAction;
use list_flow::ListAction;
use main_help_flow::MainHelpAction;
use open_flow::OpAction;
use projects::Projects;
use select_flow::render_loop;
use utils::ActionTrait;
use utils::{catch_empty_project_list, check_help_flag, check_valid_flag, get_profile_path};

const PROJECTS_DIR: &str = "Projects";
const DEPLOYS_DIR: &str = "deploys";
const OP_INCLUDE: &str = ".opinclude";

#[derive(Debug, PartialEq)]
enum ArgAction<'a> {
    ListAllProjects(ListAction),
    OpenProject(OpAction),
    MainHelp(MainHelpAction),
    CreateLayout(CreateLayout<'a>),
    AddToOpInclude(IncludeAction),
}

impl<'a> ArgAction<'a> {
    fn execute(&self) -> Result<()> {
        match self {
            Self::MainHelp(action) => action.execute(),
            Self::OpenProject(action) => action.execute(),
            Self::ListAllProjects(action) => action.execute(),
            Self::CreateLayout(action) => action.execute(),
            Self::AddToOpInclude(action) => action.execute(),
        }
    }
}

fn main() {
    if let Err(err) = run() {
        println!("{err}");
    }
}

fn run() -> Result<()> {
    let mut args = std::env::args();

    if args.len() == 1 {
        let profile_path = get_profile_path()?;
        let proj_dir = Path::new(&profile_path).join(PROJECTS_DIR);

        if !proj_dir.try_exists()? {
            // early return  as `PROJECTS_DIR` is just created and
            // will contain nothing
            return Ok(create_projects_dir()?);
        }

        let mut projects = Projects::new(proj_dir, true)?;
        catch_empty_project_list(&projects.filtered_items)?;
        render_loop(&mut projects)?;
    } else {
        // first arg is generally the program path and hence skipped here
        args.next();

        let action = process_arg_command(&mut args)?;
        action.execute()?;
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
    } else if check_valid_flag(&arg, "create")? {
        let mut create_args = CreateLayout::new();
        if let Some(iarg) = &args.next() {
            create_args.help = check_help_flag(iarg, args)?;
        }
        return Ok(ArgAction::CreateLayout(create_args));
    } else if check_valid_flag(&arg, "add")? {
        let mut include_args = IncludeAction {
            path: String::new(),
            help: false,
        };
        match args.next() {
            Some(iarg) => {
                let path = PathBuf::from(&iarg);
                if path.exists() {
                    include_args.path = iarg;
                    if let Some(iarg) = &args.next() {
                        include_args.help = check_help_flag(&iarg, args)?;
                    }
                } else {
                    include_args.help = check_help_flag(&iarg, args)?;
                }
            }
            None => return Err(Error::InvalidArgs),
        }
        return Ok(ArgAction::AddToOpInclude(include_args));
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

    // --list --help <something more>
    let mut args = ["--list".to_owned(), "--help".to_owned(), "x".to_owned()].into_iter();
    match process_arg_command(&mut args) {
        Ok(_) => assert!(false),
        Err(_) => assert!(true),
    }
}

#[test]
fn test_process_create_layout_action() {
    // --create
    let mut args = ["--create".to_owned()].into_iter();
    let act = process_arg_command(&mut args).unwrap();
    let exp = ArgAction::CreateLayout(CreateLayout::new());
    assert_eq!(act, exp);

    // --create --help
    let mut args = ["--create".to_owned()].into_iter();
    let act = process_arg_command(&mut args).unwrap();
    let exp = ArgAction::CreateLayout(CreateLayout::new());
    assert_eq!(act, exp);

    // --create --help <something more>
    let mut args = ["--list".to_owned(), "--help".to_owned(), "y".to_owned()].into_iter();
    match process_arg_command(&mut args) {
        Ok(_) => assert!(false),
        Err(_) => assert!(true),
    }
}

#[test]
fn test_process_open_action() {
    // project
    let mut args = ["project".to_owned()].into_iter();
    let act = process_arg_command(&mut args).unwrap();
    let op_args = OpAction {
        proj_name: "project".to_owned(),
        print_path: false,
        help: false,
    };
    let _exp = ArgAction::OpenProject(op_args);
    assert_eq!(act, _exp);

    // project --help
    let mut args = ["project".to_owned(), "--help".to_owned()].into_iter();
    let act = process_arg_command(&mut args).unwrap();
    let op_args = OpAction {
        proj_name: "project".to_owned(),
        print_path: false,
        help: true,
    };
    let exp = ArgAction::OpenProject(op_args);
    assert_eq!(act, exp);

    // project --help <something more>
    let mut args = ["project".to_owned(), "--help".to_owned(), "x".to_owned()].into_iter();
    match process_arg_command(&mut args) {
        Ok(_) => assert!(false),
        Err(_) => assert!(true),
    }
}

#[test]
fn test_process_open_action_print() {
    // project --print
    let mut args = ["project".to_owned(), "--print".to_owned()].into_iter();
    let act = process_arg_command(&mut args).unwrap();
    let op_args = OpAction {
        proj_name: "project".to_owned(),
        print_path: true,
        help: false,
    };
    let exp = ArgAction::OpenProject(op_args);
    assert_eq!(act, exp);

    // project --print --help
    let mut args = [
        "project".to_owned(),
        "--print".to_owned(),
        "--help".to_owned(),
    ]
    .into_iter();
    let act = process_arg_command(&mut args).unwrap();
    let op_args = OpAction {
        proj_name: "project".to_owned(),
        print_path: true,
        help: true,
    };
    let exp = ArgAction::OpenProject(op_args);
    assert_eq!(act, exp);

    // project --help --print
    let mut args = [
        "project".to_owned(),
        "--help".to_owned(),
        "--print".to_owned(),
    ]
    .into_iter();
    match process_arg_command(&mut args) {
        Ok(_) => assert!(false),
        Err(_) => assert!(true),
    }
}

#[test]
fn test_add_to_opinclude_action() {
    // --add <some valid path>
    let some_existing_path = get_profile_path().unwrap();
    let mut args = ["--add".to_owned(), some_existing_path.clone()].into_iter();
    let act = process_arg_command(&mut args).unwrap();
    let include_args = IncludeAction {
        path: some_existing_path,
        help: false,
    };
    let exp = ArgAction::AddToOpInclude(include_args);
    assert_eq!(act, exp);

    // project --add <invalid path>
    let some_existing_path = "/invalid/path".to_owned();
    let mut args = ["--add".to_owned(), some_existing_path.clone()].into_iter();
    let act = process_arg_command(&mut args).is_ok();
    assert_eq!(act, false);

    // --add --help
    let mut args = ["--add".to_owned(), "--help".to_owned()].into_iter();
    let act = process_arg_command(&mut args).unwrap();
    let include_args = IncludeAction {
        path: String::new(),
        help: true,
    };
    let exp = ArgAction::AddToOpInclude(include_args);
    assert_eq!(act, exp);
    //
    // --add <some valid path> --help
    let some_existing_path = get_profile_path().unwrap();
    let mut args = [
        "--add".to_owned(),
        some_existing_path.clone(),
        "--help".to_owned(),
    ]
    .into_iter();
    let act = process_arg_command(&mut args).unwrap();
    let include_args = IncludeAction {
        path: some_existing_path,
        help: true,
    };
    let exp = ArgAction::AddToOpInclude(include_args);
    assert_eq!(act, exp);
}
