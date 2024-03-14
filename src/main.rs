mod actions;
mod error;
mod tests;
mod utils;

use std::fs::read_to_string;
use std::path::PathBuf;

use actions::create_layout::CreateLayout;
use actions::git_status::GitStatusAction;
use actions::list_projects::ListAction;
use actions::main_help::MainHelpAction;
use actions::open_in_nvim::OpAction;
use actions::opinclude_actions::IncludeAction;
use error::{Error, Result};
use utils::constants::{DEPLOYS_DIR, OP_CONFIG, PROJECTS_DIR};
use utils::create_projects_dir;
use utils::projects::Projects;
use utils::select_ui::render_loop;
use utils::{catch_empty_project_list, check_help_flag, check_valid_flag, get_profile_path};
use utils::{ActionTrait, ShortFlag};

fn main() {
    if let Err(err) = run() {
        eprintln!("{err}");
    }
}

#[derive(Debug, Default)]
struct Config {
    projects_dir: PathBuf,
    deploys_dir: String,
    include: Vec<String>,
}

impl Config {
    fn new() -> Result<Self> {
        let home_dir = PathBuf::from(&get_profile_path()?);
        let config_file = home_dir.join(OP_CONFIG);

        let mut config = Config {
            projects_dir: home_dir.join(PROJECTS_DIR),
            deploys_dir: DEPLOYS_DIR.to_owned(),
            include: Vec::new(),
        };
        if config_file.exists() {
            for line in read_to_string(config_file)?.lines() {
                if line.starts_with('#') {
                    break;
                }
                if let Some((key, value)) = line.split_once('=') {
                    match key {
                        "projects_dir" => {
                            config.projects_dir = PathBuf::from(value);
                        }
                        "deploys_dir" => {
                            config.deploys_dir = value.to_owned();
                        }
                        "include" => {
                            config.include.push(value.to_owned());
                        }
                        _ => {}
                    }
                }
            }
        }
        Ok(config)
    }
}

#[derive(Debug, PartialEq)]
enum ArgAction<'a> {
    MainHelp(MainHelpAction),
    ListAllProjects(ListAction),
    CreateLayout(CreateLayout<'a>),
    OpenProject(OpAction),
    AddToOpConfig(IncludeAction),
    GetGitStatus(GitStatusAction),
}

impl<'a> ArgAction<'a> {
    fn execute(&self) -> Result<()> {
        let config = Config::new()?;
        match self {
            Self::MainHelp(action) => action.execute(config),
            Self::ListAllProjects(action) => action.execute(config),
            Self::CreateLayout(action) => action.execute(config),
            Self::OpenProject(action) => action.execute(config),
            Self::AddToOpConfig(action) => action.execute(config),
            Self::GetGitStatus(action) => action.execute(config),
        }
    }
}

fn run() -> Result<()> {
    let mut args = std::env::args();

    if args.len() == 1 {
        let config = Config::new()?;
        let proj_dir = config.projects_dir;

        if !proj_dir.try_exists()? {
            // early return  as `PROJECTS_DIR` is just created and
            // will contain nothing
            return create_projects_dir::start(proj_dir);
        }

        let mut projects = Projects::new(proj_dir, config.include, true)?;
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
    }
    if check_valid_flag(&arg, "create", ShortFlag::Infer)? {
        let mut create_args = CreateLayout::new();
        if let Some(iarg) = &args.next() {
            create_args.help = check_help_flag(iarg, args)?;
        }
        return Ok(ArgAction::CreateLayout(create_args));
    }
    if check_valid_flag(&arg, "add", ShortFlag::Infer)? {
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
        return Ok(ArgAction::AddToOpConfig(include_args));
    }

    if check_valid_flag(&arg, "git-status", ShortFlag::Value('g'))? {
        let mut git_status_args = GitStatusAction { help: false };
        if let Some(iarg) = &args.next() {
            git_status_args.help = check_help_flag(iarg, args)?;
        }
        return Ok(ArgAction::GetGitStatus(git_status_args));
    }

    // we go the OpenProject if no other flags are matched
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

    Ok(ArgAction::OpenProject(op_args))
}
