use jati::run::RunState as JatiRunState;
use jati::symbols::fun::FunSig;
use jati::symbols::fun::PreDefFun;
use jati::symbols::symbol_table::PreDefFunTable;
use jati::trees::types::Type;
use jati::trees::values::Value;
use uuid::uuid;
use crate::error::Error;
use crate::interpreter::RunState;

type SymaPreDefFun<'a> = PreDefFun<'a, RunState, PreDefFunTable, Error>;

const EXIT: SymaPreDefFun = SymaPreDefFun {
    name: "exit",
    uuid: uuid!("d4222c6f-608d-4c37-9b3f-95d625174272"),
    sig: FunSig::Fixed { tpe: Type::Unit, arg_types: vec![] },
    run: exit,
};

pub(crate) const PRE_DEF_FUNS: [SymaPreDefFun; 1] = [EXIT];

fn exit(_args: &[Value], run_state: &mut RunState, _symbols: &mut PreDefFunTable)
    -> Result<Value, Error> {
    run_state.request_stop();
    println!("Exiting");
    Ok(Value::Unit)
}