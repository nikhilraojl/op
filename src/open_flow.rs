use crate::error::Result;
use crate::projects::Projects;
use crate::utils::ActionTrait;

#[derive(Debug, PartialEq)]
pub struct OpAction {
    pub proj_name: String,
    pub print_path: bool,
    pub help: bool,
}

impl ActionTrait for OpAction {
    fn print_help(&self) {
        println!("Try to use one of the below commands \n");
        println!("op <project_name>            : Opens project directly in neovim");
        println!("op <project_name> --print|-p : Prints project path to stdout");
    }
    fn execute(&self, projects: &Projects) -> Result<()> {
        if self.help {
            self.print_help();
        } else if self.print_path {
            projects.print_work_dir(&self.proj_name);
        } else {
            projects.open_project_in_nvim(&self.proj_name)?;
        }
        Ok(())
    }
}
