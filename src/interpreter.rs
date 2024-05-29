use jati::parse_string;
use jati::symbols::symbol_table::PreDefFunTable;
use jati::run::RunState as JatiRunState;

use crate::error::Error;
use crate::executor::Executor;
use crate::parser::script_parser;

use crate::predef::PRE_DEF_FUNS;

pub struct Interpreter {
    symbols: PreDefFunTable,
    state: RunState,
    executor: Executor,
}

pub(crate) struct RunState {
    pub(crate) stop_requested: bool,
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
        let symbols = PreDefFunTable::new(&PRE_DEF_FUNS);
        let state = RunState::new();
        let executor = Executor::new();
        Interpreter { symbols, state, executor }
    }

    pub fn stop_requested(&self) -> bool { self.state.stop_requested }
    pub fn request_stop(&mut self) { self.state.stop_requested = true; }
    pub fn evaluate(&mut self, line: &str) -> Response {
        let raw_tree =
            match parse_string(script_parser(), line) {
                Ok(raw_tree) => { raw_tree }
                Err(error) => { return get_failure(line, error); }
            };
        let typed_tree =
            match raw_tree.into_typed(&mut self.symbols) {
                Ok(typed_tree) => { typed_tree }
                Err(error) => { return get_failure(line, error); }
            };
        self.executor.execute(typed_tree, &mut self.state);
        if line == "exit()" {  //  TODO: Replace with execution of exit function
            self.state.request_stop();
        }
        Response::Success(Success { refined_line: line.to_string() })
    }
}

impl RunState {
    pub(crate) fn new() -> RunState { RunState { stop_requested: false } }
}

impl JatiRunState for RunState {
    fn request_stop(&mut self) { self.stop_requested = true; }
    fn stop_requested(&self) -> bool { self.stop_requested }
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

