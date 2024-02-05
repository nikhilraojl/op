mod actions;
mod error;
mod tests;
mod utils;

use std::path::{Path, PathBuf};

use actions::create_layout::CreateLayout;
use actions::list_projects::ListAction;
use actions::main_help::MainHelpAction;
use actions::open_in_nvim::OpAction;
use actions::opinclude_actions::{IncludeAction, PopAction};
use error::{Error, Result};
use utils::constants::PROJECTS_DIR;
use utils::create_projects_dir;
use utils::projects::Projects;
use utils::select_ui::render_loop;
use utils::{catch_empty_project_list, check_help_flag, check_valid_flag, get_profile_path};
use utils::{ActionTrait, ShortFlag};

#[derive(Debug, PartialEq)]
enum ArgAction<'a> {
    ListAllProjects(ListAction),
    OpenProject(OpAction),
    MainHelp(MainHelpAction),
    CreateLayout(CreateLayout<'a>),
    AddToOpInclude(IncludeAction),
    PopFromOpInclude(PopAction),
}

impl<'a> ArgAction<'a> {
    fn execute(&self) -> Result<()> {
        match self {
            Self::MainHelp(action) => action.execute(),
            Self::OpenProject(action) => action.execute(),
            Self::ListAllProjects(action) => action.execute(),
            Self::CreateLayout(action) => action.execute(),
            Self::AddToOpInclude(action) => action.execute(),
            Self::PopFromOpInclude(action) => action.execute(),
        }
    }
}

fn main() {
    if let Err(err) = run() {
        eprintln!("{err}");
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
            return create_projects_dir::start();
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

    if check_valid_flag(&arg, "help", ShortFlag::Infer)? {
        return Ok(ArgAction::MainHelp(MainHelpAction));
    }

    if check_valid_flag(&arg, "list", ShortFlag::Infer)? {
        let mut list_args = ListAction::default();
        if let Some(iarg) = &args.next() {
            list_args.help = check_help_flag(iarg, args)?;
        }
        return Ok(ArgAction::ListAllProjects(list_args));
    } else if check_valid_flag(&arg, "create", ShortFlag::Infer)? {
        let mut create_args = CreateLayout::new();
        if let Some(iarg) = &args.next() {
            create_args.help = check_help_flag(iarg, args)?;
        }
        return Ok(ArgAction::CreateLayout(create_args));
    } else if check_valid_flag(&arg, "add", ShortFlag::Infer)? {
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
                        include_args.help = check_help_flag(iarg, args)?;
                    }
                } else {
                    include_args.help = check_help_flag(&iarg, args)?;
                }
            }
            None => return Err(Error::InvalidArgs),
        }
        return Ok(ArgAction::AddToOpInclude(include_args));
    } else if check_valid_flag(&arg, "pop", ShortFlag::Value('o'))? {
        let mut pop_args = PopAction { help: false };
        if let Some(iarg) = &args.next() {
            pop_args.help = check_help_flag(iarg, args)?;
        }
        return Ok(ArgAction::PopFromOpInclude(pop_args));
    } else {
        // we go the OpenProject route
        let mut op_args = OpAction {
            proj_name: arg,
            print_path: false,
            help: false,
        };

        let mut next_arg = args.next();

        if let Some(iarg) = &next_arg {
            if check_valid_flag(iarg, "print", ShortFlag::Infer)? {
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
