use std::fs::DirBuilder;

use crate::error::Result;
use crate::{projects::Projects, utils::ActionTrait};

#[derive(Debug, PartialEq)]
pub struct CreateProjLayoutAction<'a> {
    lang_types: Vec<&'a str>,
    pub help: bool,
}

impl CreateProjLayoutAction<'_> {
    pub fn new() -> Self {
        Self {
            lang_types: vec!["python", "javascript", "rust", "go", "plain_txt"],
            help: false,
        }
    }
}

impl ActionTrait for CreateProjLayoutAction<'_> {
    fn print_help(&self) {
        //TODO: Find some better way to format help text in code
        println!(
            r#"
op --create|-c : Creates Projects->language layout in home directory
                 Dirs for languages {:?} will be created"#,
            self.lang_types
        );
    }
    fn execute(&self, projects: &Projects) -> Result<()> {
        if self.help {
            self.print_help();
        } else {
            for lang in &self.lang_types {
                let path = projects.project_path.join(lang);
                if path.exists() {
                    println!("'{}' directory exists. Skipping", lang);
                } else {
                    println!("Creating {:?} directory", lang);
                    DirBuilder::new().recursive(true).create(path)?;
                }
            }
            println!("Done");
        }
        Ok(())
    }
}
