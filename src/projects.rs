use std::fmt::Display;
use std::path::PathBuf;
use walkdir::{DirEntry, WalkDir};

use crate::Result;

pub struct Projects {
    pub selected: usize,
    pub dir_items: Vec<DirEntry>,
    pub filtered_items: Vec<DirEntry>,
    // to be used when running cli command without any args
    no_arg: bool,
}

impl Projects {
    fn get_list(project_path: &PathBuf, ignore_path: &PathBuf) -> std::io::Result<Vec<DirEntry>> {
        let mut projs_vec = Vec::<DirEntry>::new();
        for entry in WalkDir::new(project_path).max_depth(2) {
            let entry = entry?;
            if entry.depth() == 2 && !entry.path().starts_with(ignore_path) {
                projs_vec.push(entry);
            }
        }
        return Ok(projs_vec);
    }
    pub fn new(project_path: PathBuf, ignore_path: PathBuf, no_arg: bool) -> Result<Self> {
        let dir_items = Self::get_list(&project_path, &ignore_path)?;
        let projects = Self {
            selected: 0,
            dir_items: dir_items.clone(),
            filtered_items: dir_items.clone(),
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
    pub fn filter_project_list(&mut self, filter_string: &String) -> Result<()> {
        self.filtered_items = self
            .dir_items
            .clone()
            .into_iter()
            .filter(|item| {
                let x = item.file_name().to_str().unwrap();
                x.starts_with(filter_string)
            })
            .collect();
        self.select_initial();
        Ok(())
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
            let dir_name = item.file_name().to_str().unwrap();
            output.push_str(dir_name);
            if idx < (self.filtered_items.len() - 1) {
                output.push('\n');
            }
        }
        write!(f, "{output}")
    }
}
