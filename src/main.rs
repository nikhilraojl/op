mod actions;
mod error;
mod tests;
mod utils;

use std::fs::read_to_string;
use std::path::PathBuf;

use actions::create_layout::CreateLayout;
use actions::fuz_find::FuzFindAction;
use actions::git_status::GitStatusAction;
use actions::list_projects::ListAction;
use actions::main_help::MainHelpAction;
use actions::open_in_nvim::OpAction;
use actions::opinclude_actions::IncludeAction;
use error::{Error, Result};
use utils::constants::{
    CONFIGFILE_EXTRA_PROJECTS_ROOT, CONFIGFILE_IGNORE_DIR, CONFIGFILE_INCLUDE,
    CONFIGFILE_PROJECTS_ROOT,
};
use utils::constants::{DEFAULT_IGNORE_DIR, DEFAULT_PROJECTS_ROOT, OP_CONFIG};
use utils::create_projects_dir;
use utils::select_ui::render_loop;
use utils::{check_help_flag, check_valid_flag, get_profile_path};
use utils::{ActionTrait, ShortFlag};

fn main() {
    if let Err(err) = run() {
        eprintln!("{err}");
    }
}

#[derive(Debug, Default)]
struct Config {
    projects_root: PathBuf,
    extra_roots: Vec<PathBuf>,
    ignore_dir: String,
    include: Vec<String>,
}

impl Config {
    fn new() -> Result<Self> {
        let home_dir = PathBuf::from(&get_profile_path()?);
        let config_file = home_dir.join(OP_CONFIG);

        let mut config = Config {
            projects_root: home_dir.join(DEFAULT_PROJECTS_ROOT),
            extra_roots: Vec::new(),
            ignore_dir: DEFAULT_IGNORE_DIR.to_owned(),
            include: Vec::new(),
        };
        if config_file.exists() {
            for line in read_to_string(config_file)?.lines() {
                if line.starts_with('#') {
                    continue;
                }
                if let Some((key, value)) = line.split_once('=') {
                    match key {
                        CONFIGFILE_PROJECTS_ROOT => {
                            config.projects_root = PathBuf::from(value);
                        }
                        CONFIGFILE_IGNORE_DIR => {
                            config.ignore_dir = value.to_owned();
                        }
                        CONFIGFILE_INCLUDE => {
                            config.include.push(value.to_owned());
                        }
                        CONFIGFILE_EXTRA_PROJECTS_ROOT => {
                            config.extra_roots.push(PathBuf::from(value));
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
    FuzFind(FuzFindAction),
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
            Self::FuzFind(action) => action.execute(config),
        }
    }
}

fn run() -> Result<()> {
    let mut args = std::env::args();

    if args.len() == 1 {
        let config = Config::new()?;
        let proj_dir = &config.projects_root;

        if !proj_dir.try_exists()? {
            return create_projects_dir::start(proj_dir);
        }
        render_loop(config)?;
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
                include_args.help = check_help_flag(&iarg, args)?;
                let path = PathBuf::from(&iarg);
                if path.exists() {
                    include_args.path = iarg;
                } else {
                    return Err(Error::NoProjectsFound);
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

    if check_valid_flag(&arg, "fuz", ShortFlag::Infer)? {
        let mut fuz_args = FuzFindAction {
            filter_string: String::new(),
            help: false,
        };

        if let Some(iarg) = &args.next() {
            fuz_args.help = check_help_flag(iarg, args)?;
            fuz_args.filter_string = iarg.to_owned();
            if args.next().is_some() {
                return Err(Error::InvalidArgs);
            }
        }
        return Ok(ArgAction::FuzFind(fuz_args));
    }

    // we go the OpenProject if no other flags are matched
    let mut op_args = OpAction {
        proj_name: arg,
        print_path: false,
        print_uri: false,
        help: false,
    };

    let mut next_arg = args.next();

    if let Some(iarg) = &next_arg {
        if check_valid_flag(iarg, "print", ShortFlag::Infer)? {
            op_args.print_path = true;
            next_arg = args.next();
        } else if check_valid_flag(iarg, "uri", ShortFlag::Infer)? {
            op_args.print_uri = true;
            next_arg = args.next();
        }
    }

    if let Some(iarg) = next_arg {
        op_args.help = check_help_flag(&iarg, args)?;
    }

    Ok(ArgAction::OpenProject(op_args))
}
