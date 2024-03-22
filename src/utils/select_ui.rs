use console::{Key, Term};

use super::projects::Projects;
use crate::error::{Error, Result};
use crate::Config;

pub fn render_loop(config: Config) -> Result<()> {
    let mut projects = Projects::new(config, true)?.catch_empty_project_list()?;
    let mut filter_string = String::new();
    let term = Term::stdout();
    term.hide_cursor()?;
    println!("{}", projects.select_ui_fmt());
    'main: loop {
        let read_key = term.read_key().expect("Failed to read key");
        if read_key == Key::ArrowUp {
            projects.select_previous();
            projects.filter_print(None, &term)?;
        }
        if read_key == Key::ArrowDown {
            projects.select_next();
            projects.filter_print(None, &term)?;
        }

        if let Key::Char(ch) = read_key {
            filter_string.push(ch);
            projects.filter_print(Some(&filter_string), &term)?;
        }
        if read_key == Key::Backspace {
            if !filter_string.is_empty() {
                filter_string.pop();
            }
            projects.filter_print(Some(&filter_string), &term)?;
        }
        if read_key == Key::Enter {
            select_project(projects)?;
            break 'main;
        }
        if read_key == Key::Escape {
            break 'main;
        }
    }
    Ok(())
}

pub fn select_project(projects: Projects) -> Result<()> {
    let projects = projects.catch_empty_project_list()?;
    let project = projects.filtered_items.get(projects.selected_idx);
    if let Some(project) = project {
        let name = project
            .file_name()
            .ok_or_else(|| Error::NoProjectsFound)?
            .to_str()
            .ok_or_else(|| Error::NoProjectsFound)?;
        projects.open_project_in_nvim(name)?;
    }
    Ok(())
}
