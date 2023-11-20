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
use std::{env::Args, iter::Peekable};
use utils::{
    catch_empty_project_list, check_help_flag, check_path_exits, check_valid_flag, get_profile_path,
};

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
        let projects = Projects::new(checked_proj_path, ignore_dir, false)?;

        let action_to_perform = process_arg_command(&mut args, &projects)?;
        match action_to_perform {
            ArgAction::MainHelp(action) => action.print_help(),
            ArgAction::OpenProject(action) => action.perform_action(&projects)?,
            ArgAction::ListAllProjects(action) => action.perform_action(&projects)?,
        }
    }

    Ok(())
}

fn process_arg_command(args: &mut Args, all_projs: &Projects) -> Result<ArgAction> {
    catch_empty_project_list(&all_projs.filtered_items)?;

    // first arg is program path and hence ignored
    args.next();

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
