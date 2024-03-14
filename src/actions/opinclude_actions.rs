use std::fs::File;
use std::io::Write;

use crate::utils::{get_config_path, ActionTrait, HelpTrait};
use crate::Config;

#[derive(PartialEq, Debug)]
pub struct IncludeAction {
    pub path: String,
    pub help: bool,
}

impl HelpTrait for IncludeAction {
    fn print_help(&self) {
        println!("op --add|-a <path>            : Adds a path to `.opinclude`");
    }
}

impl ActionTrait for IncludeAction {
    fn execute(&self, _config: Config) -> crate::error::Result<()> {
        if self.help {
            self.print_help();
        } else {
            let config_path = get_config_path()?;
            let mut config_file = File::options()
                .create(true)
                .append(true)
                .open(config_path)?;
            writeln!(&mut config_file, "include={}", self.path)?;
        }
        Ok(())
    }
}
