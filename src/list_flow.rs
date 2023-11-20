use crate::projects::Projects;
use crate::error::Result;

#[derive(Default)]
pub struct ListAction {
    pub help: bool,
}

impl ListAction {
    pub fn print_help(&self) {
        println!("op --list|-l : Prints all available projects to stdout");
    }
    pub fn perform_action(&self, projects: &Projects) -> Result<()> {
        if self.help {
            self.print_help();
        } else {
            println!("Available projects:\n");
            println!("{}", projects);
        }
        Ok(())
    }
}
