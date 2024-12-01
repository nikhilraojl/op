use crate::error::Result;
use crate::utils::{get_projects, ActionTrait, HelpTrait};
use crate::Config;

#[derive(Default, Debug, PartialEq)]
pub struct FuzFindAction {
    pub help: bool,
    pub filter_string: String,
}
impl HelpTrait for FuzFindAction {
    fn print_help(&self) {
        println!("op --fuz|-f <string_str>          : Fuzzy finds and prints projects containing string_str to stdout");
    }
}

impl ActionTrait for FuzFindAction {
    fn execute(&self, config: Config) -> Result<()> {
        if self.help {
            self.print_help();
        } else {
            let mut proj_list = get_projects(config)?.catch_empty_project_list()?;
            let fuz_list = proj_list
                .filter_project_list(&self.filter_string)
                .into_iter()
                .map(|v| {
                    v.0.file_name()
                        .unwrap_or_else(|| panic!("Failed to convert file to OsStr"))
                })
                .collect::<Vec<_>>();

            let mut output = String::new();
            for (idx, proj_name) in fuz_list.iter().enumerate() {
                output.push_str(
                    proj_name
                        .to_str()
                        .expect("Failed to convert filename to OsStr"),
                );
                if idx < fuz_list.len() - 1 {
                    output.push('\n');
                }
            }
            println!("{}", output);
        }
        Ok(())
    }
}
