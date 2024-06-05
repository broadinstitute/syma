use jati::runtime::Runtime as JatiRunState;
use jati::symbols::ops::{OpFn, OpSig};
use jati::symbols::ops::OpPreDef;
use jati::symbols::symbol_table::BasicSymbolTable;
use jati::trees::types::Type;
use jati::trees::values::Value;

use crate::error::Error;
use crate::runtime::Runtime;

type SymaPreDefFun<'a> = OpPreDef<'a, Runtime>;

const EXIT: SymaPreDefFun = SymaPreDefFun {
    name: "exit",
    sig: OpSig::Fixed { tpe: Type::Unit, arg_types: vec![] },
    func: OpFn::new(exit),
};

pub(crate) const PRE_DEF_FUNS: [SymaPreDefFun; 1] = [EXIT];

fn exit(_args: &[Value], run_state: &mut Runtime, _symbols: &mut BasicSymbolTable<Runtime>)
        -> Result<Value, Error> {
    run_state.request_stop();
    println!("Exiting");
    Ok(Value::Unit)
}