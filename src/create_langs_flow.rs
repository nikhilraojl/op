use std::fs::DirBuilder;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

use crate::error::Result;
use crate::utils::ActionTrait;
use crate::utils::HelpTrait;
use crate::OP_INCLUDE;

#[derive(Debug, PartialEq)]
pub struct CreateLanguageDirs<'a> {
    lang_types: Vec<&'a str>,
    pub help: bool,
}

impl CreateLanguageDirs<'_> {
    pub fn new() -> Self {
        Self {
            lang_types: vec!["python", "javascript", "rust", "go", "plain_txt"],
            help: false,
        }
    }
    fn create_lang_dirs(&self, path: &PathBuf) -> Result<()> {
        // create lang dirs
        for lang in &self.lang_types {
            let path = path.join(lang);
            if path.exists() {
                println!("'{}' directory exists. SKIPPING", lang);
            } else {
                println!("CREATING '{}' directory", lang);
                DirBuilder::new().recursive(true).create(path)?;
            }
        }
        // create `.opinclude` file
        let op_include_path = path.join(OP_INCLUDE);
        if let Ok(mut f) = File::options()
            .write(true)
            .create_new(true)
            .open(op_include_path)
        {
            println!("CREATING '{}' file", OP_INCLUDE);
            f.write(
                b"# include absolute paths to directories\n# lines starting with `#` are ignored",
            )?;
        } else {
            println!("'{}' directory exists. SKIPPING", OP_INCLUDE);
        };
        println!("Done");
        Ok(())
    }
}

impl HelpTrait for CreateLanguageDirs<'_> {
    fn print_help(&self) {
        println!("op --create|-c : Creates Projects->language layout in home directory\n");
        println!("- Dirs for languages {:?} will be created", self.lang_types);
    }
}

impl ActionTrait for CreateLanguageDirs<'_> {
    fn execute(&self) -> Result<()> {
        if self.help {
            self.print_help();
        } else {
            let projects = Self::get_projects()?;
            return self.create_lang_dirs(&projects.project_path);
        }
        Ok(())
    }
}
