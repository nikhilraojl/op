use std::fs::DirBuilder;
use std::path::Path;

use crate::error::Result;
use crate::utils::get_profile_path;
use console::{Key, Term};

pub fn start() -> Result<()> {
    let term = Term::stdout();
    term.hide_cursor()?;
    term.write_line("No directory named '[Pp]rojects' found")?;
    term.write_line("Would you like to create one? y/n")?;

    let read_key = term.read_key().expect("Expected some key to be pressed");
    if let Key::Char(ch) = read_key {
        if ch == 'y' || ch == 'Y' {
            let profile = get_profile_path()?;
            let profile_path = Path::new(&profile);

            let path = profile_path.join("Projects");

            DirBuilder::new().create(path)?;
            println!("Done creating 'Projects' in {:?}", profile_path);
            println!("You can now use --create option to create language dirs");
            return Ok(());
        }
    }
    println!("Nothing is created. For help check README");
    Ok(())
}
