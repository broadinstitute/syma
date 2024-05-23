use jati::parse_string;

use crate::error::Error;
use crate::parser::script_parser;

pub struct Interpreter {
    stop_requested: bool,
}

pub enum Response {
    Success(Success),
    Failure(Failure),
}

pub struct Success {
    refined_line: String,
}

pub struct Failure {
    line: String,
    pub(crate) error: Error,
}

impl Interpreter {
    pub fn new() -> Interpreter {
        Interpreter { stop_requested: false }
    }

    pub fn stop_requested(&self) -> bool { self.stop_requested }
    pub fn request_stop(&mut self) { self.stop_requested = true; }
    pub fn evaluate(&mut self, line: &str) -> Response {
        let raw_tree =
            match parse_string(script_parser(), line) {
                Ok(raw_tree) => { raw_tree }
                Err(error) => { return get_failure(line, error); }
            };
        let typed_tree = raw_tree.into_typed();
        if line == "exit()" {
            self.request_stop();
        }
        Response::Success(Success { refined_line: line.to_string() })
    }
}

fn get_failure<E>(line: &str, error: E) -> Response
    where Error: From<E> {
    let line = line.to_string();
    let error = Error::from(error);
    Response::Failure(Failure { line, error })
}

impl Response {
    pub fn line_for_history(&self) -> &str {
        match self {
            Response::Success(success) => &success.refined_line,
            Response::Failure(failure) => &failure.line
        }
    }
}

