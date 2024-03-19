use crate::error::Result;
use crate::utils::{catch_empty_project_list, get_projects, ActionTrait, HelpTrait};
use crate::Config;

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
    fn execute(&self, config: Config) -> Result<()> {
        if self.help {
            self.print_help();
        } else {
            let projects = get_projects(config)?;
            catch_empty_project_list(&projects.filtered_items)?;
            println!("{}", projects);
        }
        Ok(())
    }
}
