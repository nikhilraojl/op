use crate::error::Result;
use crate::projects::Projects;
use crate::utils::ActionTrait;

#[derive(Default, Debug, PartialEq)]
pub struct ListAction {
    pub help: bool,
}

impl ActionTrait for ListAction {
    fn print_help(&self) {
        println!("op --list|-l : Prints all available projects to stdout");
    }
    fn execute(&self, projects: &Projects) -> Result<()> {
        if self.help {
            self.print_help();
        } else {
            println!("Available projects:\n");
            println!("{}", projects);
        }
        Ok(())
    }
}
