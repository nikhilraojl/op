use crate::error::Result;
use crate::utils::{get_projects, ActionTrait, HelpTrait};
use crate::Config;

#[derive(Debug, PartialEq)]
pub struct OpAction {
    pub proj_name: String,
    pub print_path: bool,
    pub help: bool,
}
impl HelpTrait for OpAction {
    fn print_help(&self) {
        println!("Try to use one of the below commands \n");
        println!("op <project_name>            : Opens project directly in neovim");
        println!("op <project_name> --print|-p : Prints project path to stdout");
    }
}
impl ActionTrait for OpAction {
    fn execute(&self, config: Config) -> Result<()> {
        if self.help {
            self.print_help();
        } else if self.print_path {
            let projects = get_projects(config)?;
            if let Some(proj) = projects.matching_project(&self.proj_name) {
                println!("{}", proj.display());
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
