
pub struct MainHelpAction;
impl MainHelpAction {
    pub fn print_help(&self) {
        println!("\n");
        println!("Try to use one of the below commands \n");
        println!("op --list|-l                  : Prints all available projects to stdout");
        println!("op <project_name>             : Opens project directly in neovim");
        println!("op <project_name> --print|-p  : Prints project path to stdout");
    }
}
