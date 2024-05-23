use rustyline::DefaultEditor;
use rustyline::error::ReadlineError;
use crate::error::Error;
use crate::files::get_history_file;
use crate::interpreter::{Interpreter, Response};
use crate::version;

const PROMPT: &str = "SM> ";

struct ErrorResponse {
    message: String,
    stop_requested: bool
}

pub fn run_shell() -> Result<(), Error> {
    version::print_intro();
    let mut editor = DefaultEditor::new()?;
    let history_file = get_history_file()?;
    if history_file.exists() {
        println!("Loading history from {:?}.", history_file);
        editor.load_history(&history_file)?;
    } else {
        println!("No history file found at {:?}.", history_file);
    }
    let mut interpreter = Interpreter::new();
    while !interpreter.stop_requested() {
        match editor.readline(PROMPT) {
            Ok(line) => {
                let response = interpreter.evaluate(&line);
                editor.add_history_entry(response.line_for_history())?;
                match response {
                    Response::Success(_) => { println!("Success!") }
                    Response::Failure(failure) => { println!("{}", failure.error) }
                }
            }
            Err(error) => {
                let response = handle_readline_error(&error);
                println!("{}", response.message);
                if response.stop_requested {
                    interpreter.request_stop();
                }
            }
        };
    }
    println!("Saving history to {:?}.", history_file);
    editor.save_history(&history_file)?;
    println!("Goodbye!");
    Ok(())
}

fn handle_readline_error(error: &ReadlineError) -> ErrorResponse {
    match &error {
        ReadlineError::Io(io_error) => {
            let message = format!("Readline IO Error: {}", io_error);
            ErrorResponse { message, stop_requested: false }
        }
        ReadlineError::Eof => {
            let message = "Readline EOF".to_string();
            ErrorResponse { message, stop_requested: true }
        }
        ReadlineError::Interrupted => {
            let message = "Readline Interrupted".to_string();
            ErrorResponse { message, stop_requested: true }
        }
        ReadlineError::Errno(nix_error) => {
            let message = format!("Readline Nix Error: {}", nix_error);
            ErrorResponse { message, stop_requested: false }
        }
        ReadlineError::WindowResized => {
            let message = "Readline Window Resized".to_string();
            ErrorResponse { message, stop_requested: false }
        }
        _ => {
            let message = format!("Readline other error: {error}");
            ErrorResponse { message, stop_requested: false }
        }
    }
}

