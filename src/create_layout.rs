use std::fs::DirBuilder;
use std::fs::File;
use std::io::Write;

use crate::error::Result;
use crate::utils::get_project_dir;
use crate::utils::ActionTrait;
use crate::utils::HelpTrait;
use crate::{OP_INCLUDE, PROJECTS_DIR};

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
    fn create_lang_dirs(&self) -> Result<()> {
        let path = get_project_dir()?;
        let recurse = !path.exists();
        if recurse {
            println!("Creating '{PROJECTS_DIR}' directory");
        }
        // create lang dirs
        for lang in &self.lang_types {
            let path = path.join(lang);
            if path.exists() {
                println!("'{PROJECTS_DIR}/{lang}' directory already exists. SKIPPING");
            } else {
                println!("Creating '{PROJECTS_DIR}/{lang}' directory");
                DirBuilder::new().recursive(recurse).create(path)?;
            }
        }
        // create `.opinclude` file. `path` is supposed to exist at this point
        let op_include_path = path.join(OP_INCLUDE);
        if let Ok(mut f) = File::options()
            .write(true)
            .create_new(true)
            .open(op_include_path)
        {
            println!("Creating '{PROJECTS_DIR}/{OP_INCLUDE}' file");
            f.write(
                b"# include absolute paths to directories\n# lines starting with `#` are ignored",
            )?;
        } else {
            println!("'{OP_INCLUDE}' directory exists. SKIPPING");
        };
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
    fn execute(&self) -> Result<()> {
        if self.help {
            self.print_help();
        } else {
            return self.create_lang_dirs();
        }
        Ok(())
    }
}
