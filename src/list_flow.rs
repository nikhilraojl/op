use crate::error::Result;
use crate::utils::{catch_empty_project_list, ActionTrait, HelpTrait};

#[derive(Default, Debug, PartialEq)]
pub struct ListAction {
    pub help: bool,
}
impl HelpTrait for ListAction {
    fn print_help(&self) {
        println!("op --list|-l : Prints all available projects to stdout");
    }
}

impl ActionTrait for ListAction {
    fn execute(&self) -> Result<()> {
        if self.help {
            self.print_help();
        } else {
            let projects = Self::get_projects()?;
            catch_empty_project_list(&projects.filtered_items)?;
            println!("Available projects:\n");
            println!("{}", projects);
        }
        Ok(())
    }
}
