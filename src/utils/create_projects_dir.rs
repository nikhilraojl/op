use std::{fs::DirBuilder, path::Path};

use crate::error::Result;
use console::{Key, Term};

pub fn start(path: &Path) -> Result<()> {
    let term = Term::stdout();
    term.hide_cursor()?;
    term.write_line("No directory named '[Pp]rojects' found")?;
    term.write_line("Would you like to create one? y/n")?;

    let read_key = term.read_key().expect("Expected some key to be pressed");
    if let Key::Char(ch) = read_key {
        if ch == 'y' || ch == 'Y' {
            DirBuilder::new().create(path)?;
            println!("Done creating {:?}", path.to_string_lossy());
            println!("You can now use --create option to create language dirs");
            return Ok(());
        }
    }
    println!("Nothing is created. For help check README");
    Ok(())
}
