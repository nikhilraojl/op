use console::Term;
use std::path::PathBuf;
use std::{fmt::Display, process::Command};
use walkdir::WalkDir;

use crate::error::Error;
use crate::utils::get_additional_paths;
use crate::{Result, DEPLOYS_DIR};

#[derive(Debug)]
pub struct Projects {
    pub project_path: PathBuf,
    pub selected: usize,
    pub dir_items: Vec<PathBuf>,
    pub filtered_items: Vec<PathBuf>,
    // to be used when running cli command without any args
    no_arg: bool,
}

impl Projects {
    fn get_list(project_path: &PathBuf, ignore_path: &PathBuf) -> std::io::Result<Vec<PathBuf>> {
        let mut projs_vec = Vec::<PathBuf>::new();
        for entry in WalkDir::new(project_path).max_depth(2) {
            let entry = entry?;
            if entry.depth() == 2 && !entry.path().starts_with(ignore_path) {
                projs_vec.push(entry.into_path());
            }
        }
        return Ok(projs_vec);
    }
    pub fn new(project_path: PathBuf, no_arg: bool) -> Result<Self> {
        let ignore_path = project_path.join(DEPLOYS_DIR);
        let mut include_paths = get_additional_paths(&project_path);
        let mut dir_items = Self::get_list(&project_path, &ignore_path)?;
        dir_items.append(&mut include_paths);
        let projects = Self {
            project_path,
            selected: 0,
            filtered_items: dir_items.clone(),
            dir_items,
            no_arg,
        };
        Ok(projects)
    }
    fn select_initial(&mut self) {
        self.selected = 0;
    }
    pub fn select_next(&mut self) {
        if self.selected < (self.filtered_items.len() - 1) {
            self.selected += 1;
        }
    }
    pub fn select_previous(&mut self) {
        if self.selected > 0 {
            self.selected -= 1;
        }
    }
    pub fn filter_project_list(&mut self, filter_string: &String) -> Vec<PathBuf> {
        self.select_initial();
        return self
            .dir_items
            .clone()
            .into_iter()
            .filter(|item| {
                let mut result = false;
                if let Some(os_name) = item.file_name() {
                    let x = os_name.to_str().unwrap();
                    result = x.starts_with(filter_string)
                }
                return result;
            })
            .collect();
    }
    pub fn filter_print(&mut self, filter_string: Option<&String>, term: &Term) -> Result<()> {
        term.clear_to_end_of_screen()?;
        term.clear_last_lines(self.filtered_items.len())?;
        if let Some(filter_string) = filter_string {
            term.clear_last_lines(1)?;
            println!("Find: {filter_string}");
            self.filtered_items = self.filter_project_list(filter_string);
        }
        if self.filtered_items.len() > 0 {
            println!("{}", self);
        }
        Ok(())
    }
    pub fn matching_project(&self, project_name: &str) -> Option<&PathBuf> {
        for proj in &self.filtered_items {
            let matching_project = proj.ends_with(project_name);
            if matching_project {
                return Some(proj);
            }
        }
        return None;
    }

    pub fn open_project_in_nvim(&self, project_name: &str) -> Result<()> {
        if let Some(proj) = self.matching_project(project_name) {
            println!("Opening project {:?}", proj.file_name());

            std::env::set_current_dir(proj)?;

            let mut nvim_process = Command::new("nvim");
            nvim_process.arg(".");
            nvim_process.status()?;
            println!("Closing project {:?}", proj.file_name());
            std::process::exit(0);
        } else {
            if self.filtered_items.len() == 0 {
                return Err(Error::NoProjectsFound);
            }
            println!("No matching projects found. Only below projects are available");
            println!("{self}");
            Ok(())
        }
    }

    pub fn print_work_dir(&self, project_name: &str) {
        if let Some(proj) = self.matching_project(project_name) {
            println!("{}", proj.display());
        } else {
            println!("No matching projects found. Couldn't switch to project dir'");
        }
    }
}

impl Display for Projects {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut output = String::new();
        for (idx, item) in self.filtered_items.iter().enumerate() {
            if self.selected == idx && self.no_arg {
                output.push_str(">> ");
            } else {
                output.push_str("   ");
            }
            if let Some(dir_name) = item.file_name() {
                let name = dir_name.to_str().unwrap();
                output.push_str(name);
                if idx < (self.filtered_items.len() - 1) {
                    output.push('\n');
                }
            }
        }
        write!(f, "{output}")
    }
}
