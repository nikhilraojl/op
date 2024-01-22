use std::fs::File;
use std::io::Write;

use crate::error::Error;
use crate::utils::constants::OP_INCLUDE;
use crate::utils::{get_project_dir, ActionTrait, HelpTrait};

#[derive(PartialEq, Debug)]
pub struct IncludeAction {
    pub path: String,
    pub help: bool,
}

impl HelpTrait for IncludeAction {
    fn print_help(&self) {
        println!("op --add|-a <path>           : Adds path to `.opinclude`");
    }
}

impl ActionTrait for IncludeAction {
    fn execute(&self) -> crate::error::Result<()> {
        if self.help {
            self.print_help();
        } else {
            let projects_dir = get_project_dir()?;
            if !projects_dir.exists() {
                return Err(Error::NoProjectsFound);
            }
            let mut file_opinclude = File::options()
                .create(true)
                .append(true)
                .open(projects_dir.join(OP_INCLUDE))?;
            writeln!(&mut file_opinclude, "{}", self.path)?;
        }
        Ok(())
    }
}
