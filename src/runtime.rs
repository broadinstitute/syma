use std::collections::BTreeMap;
use jati::runtime::Runtime as JatiRuntime;
use jati::symbols::ops::{OpFn, OpKey};
use jati::symbols::symbol_table::BasicSymbolTable;
use jati::symbols::var::VarKey;
use jati::trees::values::Value;
use crate::error::Error;

pub(crate) struct Runtime {
    stop_requested: bool,
    var_values: BTreeMap<VarKey, Value>,
    op_funcs: BTreeMap<OpKey, OpFn<Self>>,
}

impl Runtime {
    pub(crate) fn new() -> Self {
        let stop_requested = false;
        let var_values: BTreeMap<VarKey, Value> = BTreeMap::new();
        let op_funcs: BTreeMap<OpKey, OpFn<Self>> = BTreeMap::new();
        Self { stop_requested, var_values, op_funcs }
    }
}

impl JatiRuntime for Runtime {
    type S = BasicSymbolTable<Self>;
    type E = Error;

    fn request_stop(&mut self) { self.stop_requested = true; }
    fn stop_requested(&self) -> bool { self.stop_requested }

    fn set_var_value(&mut self, key: &VarKey, value: Value) -> Result<(), Self::E> {
        self.var_values.insert(*key, value);
        Ok(())
    }

    fn get_var_value(&self, key: &VarKey) -> Result<Value, Self::E> {
        let value =
            self.var_values.get(key).cloned()
                .ok_or(Error::from("Invalid internal variable reference."))?;
        Ok(value)
    }

    fn apply_func(&mut self, key: &OpKey, args: &[Value], symbols: &mut Self::S)
        -> Result<Value, Self::E> {
        let func =
            self.op_funcs.get(key)
                .ok_or(Error::from("Invalid internal function reference."))?;
        (func.func)(args, self, symbols)
    }
}

