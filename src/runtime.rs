use jati::runtime::Runtime as JatiRuntime;
use jati::symbols::ops::{OpFn, OpKey};
use jati::symbols::symbol_table::BasicSymbolTable;
use jati::symbols::var::VarKey;
use jati::trees::values::Value;
use crate::error::Error;

pub(crate) struct Runtime {
    stop_requested: bool,
}

impl Runtime {
    pub(crate) fn new() -> Self {
        Self { stop_requested: false }
    }
}

impl JatiRuntime for Runtime {
    type S = BasicSymbolTable<Self>;
    type E = Error;

    fn request_stop(&mut self) { self.stop_requested = true; }
    fn stop_requested(&self) -> bool { self.stop_requested }

    fn set_var_value(&mut self, key: &VarKey, value: Value) -> Result<(), Self::E> {
        todo!()
    }

    fn get_var_value(&self, key: &VarKey) -> Result<Value, Self::E> {
        todo!()
    }

    fn get_op_func(&self, key: &OpKey) -> Result<OpFn<Self>, Self::E> {
        todo!()
    }
}

