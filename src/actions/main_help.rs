use crate::error::Result;
use crate::utils::{ActionTrait, HelpTrait};

#[derive(Debug, PartialEq)]
pub struct MainHelpAction;
impl HelpTrait for MainHelpAction {
    fn print_help(&self) {
        //TODO: Find some better way to format help text in code
        println!("\n");
        println!("Try to use one of the below commands \n");
        println!("op --list|-l                  : Prints all available projects to stdout");
        println!(
            "op --create|-c                : Creates Projects->language layout in home directory"
        );
        println!("op <project_name>             : Opens project directly in neovim");
        println!("op <project_name> --print|-p  : Prints project path to stdout");
        println!("op --add|-a <path>            : Adds a path to `.opinclude`");
        println!("op --pop|-o                   : Pops last path from `.opinclude`");
    }
}
impl ActionTrait for MainHelpAction {
    fn execute(&self) -> Result<()> {
        self.print_help();
        Ok(())
    }
}
