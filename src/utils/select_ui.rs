use console::{Key, Term};

use super::projects::Projects;
use crate::error::{Error, Result};
use crate::utils::catch_empty_project_list;

pub fn render_loop(projects: &mut Projects) -> Result<()> {
    let mut filter_string = String::new();
    let term = Term::stdout();
    term.hide_cursor()?;
    println!("{projects}");
    'main: loop {
        let read_key = term.read_key().unwrap();
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

pub fn select_project(projects: &mut Projects) -> Result<()> {
    catch_empty_project_list(&projects.filtered_items)?;
    let project = projects.filtered_items.get(projects.selected);
    if let Some(project) = project {
        let name = project.to_str().ok_or_else(|| Error::NoProjectsFound)?;
        projects.open_project_in_nvim(name)?;
    }
    Ok(())
}
