use rustyline::DefaultEditor;
use rustyline::error::ReadlineError;
use crate::error::Error;
use crate::files::get_history_file;

const PROMPT: &str = "SM> ";
const VERSION: Option<&str> = option_env!("CARGO_PKG_VERSION");

pub fn run_shell() -> Result<(), Error> {
    match VERSION {
        None => { println!("Welcome to Syma!"); }
        Some(version) => { println!("Welcome to Syma Version {version}!"); }
    }
    let mut editor = DefaultEditor::new()?;
    let history_file = get_history_file()?;
    if history_file.exists() {
        println!("Loading history from {:?}.", history_file);
        editor.load_history(&history_file)?;
    } else {
        println!("No history file found at {:?}.", history_file);
    }
    loop {
        match editor.readline(PROMPT) {
            Ok(line) => {
                editor.add_history_entry(line.as_str())?;
                if line == "exit" {
                    break;
                }
            }
            Err(error) => {
                handle_readline_error(&error);
            }
        };
    }
    println!("Saving history to {:?}.", history_file);
    editor.save_history(&history_file)?;
    println!("Goodbye!");
    Ok(())
}

fn handle_readline_error(error: &ReadlineError) {
    match &error {
        ReadlineError::Io(io_error) => {
            println!("Readline IO Error: {}", io_error)
        }
        ReadlineError::Eof => {
            println!("Readline EOF")
        }
        ReadlineError::Interrupted => {
            println!("Readline Interrupted")
        }
        ReadlineError::Errno(nix_error) => {
            println!("Readline Nix Error: {}", nix_error)
        }
        ReadlineError::WindowResized => {
            println!("Readline Window Resized")
        }
        _ => {
            println!("Readline other error: {error}")
        }
    }
}

