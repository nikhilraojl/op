use std::fs::DirBuilder;

use crate::error::Result;
use crate::utils::ActionTrait;
use crate::utils::HelpTrait;
use crate::Config;

#[derive(Debug, PartialEq)]
pub struct CreateLayout<'a> {
    lang_types: Vec<&'a str>,
    pub help: bool,
}

impl CreateLayout<'_> {
    pub fn new() -> Self {
        Self {
            lang_types: vec!["python", "javascript", "rust", "go", "plain_txt"],
            help: false,
        }
    }
    fn create_lang_dirs(&self, config: Config) -> Result<()> {
        let path = config.projects_root;
        let recurse = !path.exists();
        if recurse {
            println!("Creating directory '{}'", path.to_string_lossy());
        }
        // create lang dirs
        for lang in &self.lang_types {
            let path = path.join(lang);
            if path.exists() {
                println!("'{lang}' sub-directory already exists. SKIPPING");
            } else {
                println!("Creating sub-directory'{lang}'");
                DirBuilder::new().recursive(recurse).create(path)?;
            }
        }
        println!("Done");
        Ok(())
    }
}

impl HelpTrait for CreateLayout<'_> {
    fn print_help(&self) {
        println!("op --create|-c : Creates Projects->language layout in home directory\n");
        println!("- Dirs for languages {:?} will be created", self.lang_types);
    }
}

impl ActionTrait for CreateLayout<'_> {
    fn execute(&self, config: Config) -> Result<()> {
        if self.help {
            self.print_help();
        } else {
            return self.create_lang_dirs(config);
        }
        Ok(())
    }
}
