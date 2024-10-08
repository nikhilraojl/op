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
        let read_key_raw = term.read_key_raw().expect("Failed to read key");
        match read_key_raw {
            Key::ArrowUp => {
                projects.select_previous();
                projects.filter_print(None, &term)?;
            }
            Key::ArrowDown => {
                projects.select_next();
                projects.filter_print(None, &term)?;
            }
            Key::Char(ch) => match ch {
                // clear content if ctrl+backspace is pressed
                // NOTE: isn't tested for cmd or super keys
                '\u{007f}' => {
                    filter_string.clear();
                    projects.filter_print(Some(&filter_string), &term)?;
                }

                // move vertically when J/K is received instead of filtering
                'J' => {
                    projects.select_next();
                    projects.filter_print(None, &term)?;
                }
                'K' => {
                    projects.select_previous();
                    projects.filter_print(None, &term)?;
                }

                // do search and filter
                _ => {
                    filter_string.push(ch);
                    projects.filter_print(Some(&filter_string), &term)?;
                }
            },
            Key::Backspace => {
                if !filter_string.is_empty() {
                    filter_string.pop();
                }
                projects.filter_print(Some(&filter_string), &term)?;
            }
            Key::Enter => {
                select_project(projects)?;
                break 'main;
            }
            Key::Escape => {
                break 'main;
            }
            _ => {}
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
