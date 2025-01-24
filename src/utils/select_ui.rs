use console::{Key, Term};

use super::projects::Projects;
use crate::error::{Error, Result};
use crate::Config;

// TODO: move select UI related functions from projects to here
fn filter_print(
    projects: &mut Projects,
    filter_string: Option<&String>,
    term: &Term,
) -> Result<()> {
    let lines_to_clear = match projects.buffer_rows < projects.filtered_items.len() {
        true => projects.buffer_rows + 1,
        false => projects.filtered_items.len(),
    };
    term.clear_to_end_of_screen()?;
    term.clear_last_lines(lines_to_clear)?;

    if let Some(filter_string) = filter_string {
        term.clear_last_lines(1)?; // this is to clear the previous `Find: <>`
        println!("Find: {filter_string}");
        projects.select_initial();
        projects.filtered_items = projects
            .filter_project_list(filter_string)
            .into_iter()
            .map(|v| v.0.to_owned())
            .collect();
    }

    if !projects.filtered_items.is_empty() {
        println!("{}", projects.select_ui_fmt());
    }
    Ok(())
}

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
                filter_print(&mut projects, None, &term)?;
            }
            Key::ArrowDown => {
                projects.select_next();
                filter_print(&mut projects, None, &term)?;
            }
            Key::Char(ch) => match ch {
                // clear content if ctrl+backspace is pressed
                // NOTE: isn't tested for cmd or super keys
                '\u{007f}' => {
                    filter_string.clear();
                    filter_print(&mut projects, Some(&filter_string), &term)?;
                }

                // move vertically when J/K is received instead of filtering
                'J' => {
                    projects.select_next();
                    filter_print(&mut projects, None, &term)?;
                }
                'K' => {
                    projects.select_previous();
                    filter_print(&mut projects, None, &term)?;
                }

                // do search and filter
                _ => {
                    filter_string.push(ch);
                    filter_print(&mut projects, Some(&filter_string), &term)?;
                }
            },
            Key::Backspace => {
                if !filter_string.is_empty() {
                    filter_string.pop();
                }
                filter_print(&mut projects, Some(&filter_string), &term)?;
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
    let project = projects
        .filtered_items
        .get(projects.selected_idx)
        .ok_or(Error::NoProjectsFound)?;
    projects.open_project_in_nvim(project)?;
    Ok(())
}
