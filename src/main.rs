mod error;
mod list_flow;
mod open_flow;
mod projects;
mod select_flow;
mod utils;

use error::{Error, Result};
use list_flow::ListArgs;
use open_flow::OpArgs;
use projects::Projects;
use select_flow::render_loop;
use std::env::Args;
use std::path::Path;
use utils::{
    catch_empty_project_list, check_no_next_arg, check_path_exits, check_valid_flag,
    get_profile_path,
};

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

    if args.len() == 1 {
        let mut projects = Projects::new(checked_proj_path, ignore_dir, true)?;
        render_loop(&mut projects)?;
    } else {
        let projects = Projects::new(checked_proj_path, ignore_dir, false)?;

        match process_arg_command(&mut args, &projects)? {
            ArgAction::OpenProject(args) => args.perform_action(&projects)?,
            ArgAction::ListAllProjects(args) => args.perform_action(&projects),
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
