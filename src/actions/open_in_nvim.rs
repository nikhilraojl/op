use std::process::{Command, Stdio};

use crate::error::Result;
use crate::utils::{get_projects, ActionTrait, HelpTrait};
use crate::Config;

#[derive(Debug, PartialEq)]
pub struct OpAction {
    pub proj_name: String,
    pub print_path: bool,
    pub print_uri: bool,
    pub help: bool,
}
impl HelpTrait for OpAction {
    fn print_help(&self) {
        println!("Try to use one of the below commands \n");
        println!("op <project_name>            : Opens project directly in neovim");
        println!("op <project_name> --print|-p : Prints project path to stdout");
        println!("op <project_name> --uri|-u   : Prints remote uri path to stdout");
    }
}
impl ActionTrait for OpAction {
    fn execute(&self, config: Config) -> Result<()> {
        if self.help {
            self.print_help();
        } else if self.print_path {
            let projects = get_projects(config)?;
            if let Some(proj) = projects.print_project_path(&self.proj_name) {
                println!("{}", proj.trim());
            } else {
                eprintln!("No matching projects found.");
            }
        } else if self.print_uri {
            let projects = get_projects(config)?;
            if let Some(proj) = projects.matching_project(&self.proj_name) {
                let mut git = Command::new("git");
                let output = git
                    .arg("-C")
                    .arg(proj)
                    .arg("config")
                    .arg("--get")
                    .arg("remote.origin.url")
                    .stdout(Stdio::piped())
                    .output()
                    .expect("git should be installed");

                let formatted_uri = String::from_utf8_lossy(&output.stdout)
                    .replace(':', "/")
                    .replace("git@", "https://")
                    .replace(".git", "");
                print!("{}", formatted_uri);
            } else {
                println!("No matching projects found. Couldn't switch to project dir'");
            }
        } else {
            let projects = get_projects(config)?;
            projects.open_project_in_nvim(&self.proj_name)?;
        }
        Ok(())
    }
}
