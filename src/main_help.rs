use crate::error::Result;
use crate::{projects::Projects, utils::ActionTrait};

#[derive(Debug, PartialEq)]
pub struct MainHelpAction;
impl ActionTrait for MainHelpAction {
    fn print_help(&self) {
        println!("\n");
        println!("Try to use one of the below commands \n");
        println!("op --list|-l                  : Prints all available projects to stdout");
        println!(
            "op --create|-c                : Creates Projects->language layout in home directory"
        );
        println!("op <project_name>             : Opens project directly in neovim");
        println!("op <project_name> --print|-p  : Prints project path to stdout");
    }
    fn execute(&self, _projects: &Projects) -> Result<()> {
        self.print_help();
        Ok(())
    }
}
