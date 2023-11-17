use crate::projects::Projects;

#[derive(Default)]
pub struct ListArgs {
    pub list: bool,
    pub help: bool,
}

impl ListArgs {
    pub fn print_help(&self) {
        println!("op --list|-l : Prints all available projects to stdout");
    }
    pub fn perform_action(&self, projects: &Projects) {
        if self.help {
            self.print_help();
        } else {
            println!("Available projects:\n");
            println!("{}", projects);
        }
    }
}
