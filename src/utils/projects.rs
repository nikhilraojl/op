use console::Term;
use std::path::PathBuf;
use std::process::Command;
use walkdir::WalkDir;

use super::validate_paths;
use crate::error::Error;
use crate::{Config, Result};

#[derive(Debug)]
pub struct Projects {
    pub selected_idx: usize,
    pub dir_items: Vec<PathBuf>,
    pub filtered_items: Vec<PathBuf>,
    // to be used for selecting ui i.e running command without any args
    cli_no_arg: bool,
    // for buffered stdout
    buffer_rows: usize,
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
        Ok(projs_vec)
    }

    pub fn new(config: Config, cli_no_arg: bool) -> Result<Self> {
        let project_root = config.projects_root;

        let mut include_paths = validate_paths(config.include);
        // from configuration `project_root`
        let ignore_path = project_root.join(&config.ignore_dir);
        let mut dir_items = Self::get_list(&project_root, &ignore_path)?;

        // from the configuration `extra_project_root`s
        for extra_project_root in config.extra_roots {
            let ignore_path = extra_project_root.join(&config.ignore_dir);
            dir_items.extend(Self::get_list(&extra_project_root, &ignore_path)?);
        }

        // from the configuration `include`s
        dir_items.append(&mut include_paths);

        dir_items.sort_by(|a, b| {
            a.file_name()
                .expect("Failed to get filename OsStr")
                .partial_cmp(b.file_name().expect("Failed to get filename OsStr"))
                .expect("Failed to sort `Projects` vec")
        });
        let projects = Self {
            selected_idx: 0,
            filtered_items: dir_items.clone(),
            dir_items,
            cli_no_arg,
            buffer_rows: 10,
        };
        Ok(projects)
    }

    pub fn catch_empty_project_list(self) -> Result<Self> {
        if self.filtered_items.is_empty() {
            Err(Error::NoProjectsFound)
        } else {
            Ok(self)
        }
    }

    pub fn filter_project_list(&mut self, filter_string: &String) -> Vec<PathBuf> {
        self.dir_items
            .clone()
            .into_iter()
            .filter(|item| {
                let mut result = false;
                if let Some(os_name) = item.file_name() {
                    let x = os_name.to_str().expect("Failed to convert OsStr to str");
                    result = x.starts_with(filter_string)
                }
                result
            })
            .collect()
    }

    pub fn matching_project(&self, project_name: &str) -> Option<&PathBuf> {
        for proj in &self.filtered_items {
            let matching_project = proj.ends_with(project_name);
            if matching_project {
                return Some(proj);
            }
        }
        None
    }

    fn select_initial(&mut self) {
        self.selected_idx = 0;
    }
    pub fn select_next(&mut self) {
        if self.selected_idx < (self.filtered_items.len() - 1) {
            self.selected_idx += 1;
        }
    }

    pub fn select_previous(&mut self) {
        if self.selected_idx > 0 {
            self.selected_idx -= 1;
        }
    }

    pub fn display_fmt(&self, from: usize, upto: usize) -> String {
        let mut output = String::new();
        for (idx, item) in self.filtered_items[from..upto].iter().enumerate() {
            if self.cli_no_arg {
                if idx == self.selected_idx - from {
                    output.push_str(">> ");
                } else {
                    output.push_str("   ");
                }
            }
            if let Some(dir_name) = item.file_name() {
                let name = dir_name.to_str().expect("Failed to convert OsStr to str");
                output.push_str(name);
                if idx < (self.filtered_items.len() - 1) {
                    output.push('\n');
                }
            }
        }
        output
    }

    pub fn select_ui_fmt(&self) -> String {
        let (from, upto) = match self.buffer_rows > self.filtered_items.len() {
            true => (0, self.filtered_items.len()),
            false => {
                let overflow_at_start =
                    self.filtered_items.len() - self.selected_idx < self.buffer_rows;
                let overflow_at_end =
                    self.selected_idx + self.buffer_rows < self.filtered_items.len();

                let from = match overflow_at_start {
                    true => self.filtered_items.len() - self.buffer_rows,
                    false => self.selected_idx,
                };
                let upto = match overflow_at_end {
                    true => self.selected_idx + self.buffer_rows,
                    false => self.filtered_items.len(),
                };
                (from, upto)
            }
        };
        self.display_fmt(from, upto)
    }

    pub fn filter_print(&mut self, filter_string: Option<&String>, term: &Term) -> Result<()> {
        let lines_to_clear = match self.buffer_rows < self.filtered_items.len() {
            true => self.buffer_rows + 1,
            false => self.filtered_items.len(),
        };
        term.clear_to_end_of_screen()?;
        term.clear_last_lines(lines_to_clear)?;

        if let Some(filter_string) = filter_string {
            term.clear_last_lines(1)?; // this is to clear the previous `Find: <>`
            println!("Find: {filter_string}");
            self.select_initial();
            self.filtered_items = self.filter_project_list(filter_string);
        }

        if !self.filtered_items.is_empty() {
            // Display implementation kicks in here
            println!("{}", self.select_ui_fmt());
        }
        Ok(())
    }

    pub fn open_project_in_nvim(&self, project_name: &str) -> Result<()> {
        if let Some(proj) = self.matching_project(project_name) {
            println!("Opening project {:?}", project_name);

            std::env::set_current_dir(proj)?;

            let mut nvim_process = Command::new("nvim");
            nvim_process.arg(".");
            nvim_process.status()?;
            println!("Closing project {:?}", project_name);
            std::process::exit(0);
        }

        if self.filtered_items.is_empty() {
            return Err(Error::NoProjectsFound);
        }

        println!("No matching projects found. Only below projects are available");
        println!("{}", self.display_fmt(0, self.filtered_items.len()));
        Ok(())
    }
}
