use jati::runtime::Runtime as JatiRuntime;
use jati::symbols::symbol_table::BasicSymbolTable;

use crate::error::Error;

pub(crate) struct Runtime {
    stop_requested: bool,
}

impl Runtime {
    pub(crate) fn new() -> Self {
        let stop_requested = false;
        Self { stop_requested }
    }
}

impl JatiRuntime for Runtime {
    type S = BasicSymbolTable<Self>;
    type E = Error;

    fn request_stop(&mut self) { self.stop_requested = true; }
    fn stop_requested(&self) -> bool { self.stop_requested }
}

