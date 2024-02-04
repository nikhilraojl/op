use std::fs::write;
use std::fs::{read_to_string, File};
use std::io::Write;

use crate::error::Error;
use crate::utils::constants::OP_INCLUDE;
use crate::utils::{get_project_dir, ActionTrait, HelpTrait};

#[derive(PartialEq, Debug)]
pub struct IncludeAction {
    pub path: String,
    pub help: bool,
}

impl HelpTrait for IncludeAction {
    fn print_help(&self) {
        println!("op --add|-a <path>            : Adds a path to `.opinclude`");
    }
}

impl ActionTrait for IncludeAction {
    fn execute(&self) -> crate::error::Result<()> {
        if self.help {
            self.print_help();
        } else {
            let projects_dir = get_project_dir()?;
            if !projects_dir.exists() {
                return Err(Error::NoProjectsFound);
            }
            let mut file_opinclude = File::options()
                .create(true)
                .append(true)
                .open(projects_dir.join(OP_INCLUDE))?;
            writeln!(&mut file_opinclude, "{}", self.path)?;
        }
        Ok(())
    }
}

#[derive(PartialEq, Debug)]
pub struct PopAction {
    pub help: bool,
}

impl HelpTrait for PopAction {
    fn print_help(&self) {
        println!("op --pop|-o                   : Pops last path from `.opinclude`");
    }
}

impl ActionTrait for PopAction {
    fn execute(&self) -> crate::error::Result<()> {
        if self.help {
            self.print_help();
        } else {
            // A naive implementation of removing last line form `OP_INCLUDE` file
            let opinclude_file = get_project_dir()?.join(OP_INCLUDE);
            if !opinclude_file.exists() {
                return Err(Error::NoProjectsFound);
            }
            let file_buf = read_to_string(&opinclude_file)?;
            let mut lines = file_buf.lines().peekable();
            if lines.clone().count() == 0 {
                println!("Nothing to pop");
                return Ok(())
            }

            let mut popped_line = "";
            let mut bytes_to_keep = 0;
            while let Some(line) = lines.next() {
                if lines.peek().is_some() {
                    bytes_to_keep += line.len();
                    bytes_to_keep += 1;
                } else {
                    popped_line = line;
                }
            }

            print!("Are you sure you want to drop '{popped_line}'? ");
            std::io::stdout().flush()?;
            let mut answer = String::new();
            let stdin = std::io::stdin();
            stdin.read_line(&mut answer)?;

            match answer.trim_end() {
                "y" | "Y" | "Yes" | "yes" => {
                    // buffer containing everything excluding last line for 
                    // writing back to file
                    let buf = &file_buf[0..bytes_to_keep];
                    write(&opinclude_file, buf)?;
                    println!("Dropped");
                    return Ok(());
                }
                _ => {
                    println!("Skipping");
                    return Ok(());
                }
            }
        }
        Ok(())
    }
}
