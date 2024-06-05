use jati::parse_string;
use jati::runtime::Runtime as JatiRunState;
use jati::symbols::symbol_table::BasicSymbolTable;
use jati::trees::execute::{Executor, SimpleExecutor};

use crate::error::Error;
use crate::parser::script_parser;
use crate::predef::PRE_DEF_FUNS;
use crate::runtime::Runtime;

pub struct Interpreter {
    symbols: BasicSymbolTable<Runtime>,
    runtime: Runtime,
    executor: SimpleExecutor<Runtime>
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
    pub fn new() -> Result<Interpreter, Error> {
        let symbols =
            BasicSymbolTable::<Runtime>::with_predef_funs(&PRE_DEF_FUNS)?;
        let runtime = Runtime::new();
        let executor = SimpleExecutor::<Runtime>::new();
        Ok(Interpreter { symbols, runtime, executor })
    }

    pub fn stop_requested(&self) -> bool { self.runtime.stop_requested() }
    pub fn request_stop(&mut self) { self.runtime.request_stop(); }
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
        match self.executor.execute(&typed_tree, &mut self.runtime, &mut self.symbols) {
            Ok(value) => {
                println!("{}", value);
                if line == "exit()" {  //  TODO: Replace with execution of exit function
                    self.runtime.request_stop();
                }
                Response::Success(Success { refined_line: line.to_string() })
            }
            Err(error) => {
                get_failure(line, error)
            }
        }
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

