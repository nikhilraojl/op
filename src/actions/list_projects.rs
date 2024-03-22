use crate::error::Result;
use crate::utils::{get_projects, ActionTrait, HelpTrait};
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
            let projects = get_projects(config)?.catch_empty_project_list()?;
            println!("{}", projects.display_fmt(0, projects.filtered_items.len()));
        }
        Ok(())
    }
}
