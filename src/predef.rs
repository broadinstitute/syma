use jati::symbols::fun::FunSig;
use jati::symbols::symbol_table::PreDefFun;
use jati::trees::types::Type;
use uuid::uuid;

const EXIT: PreDefFun = PreDefFun {
    name: "exit",
    uuid: uuid!("d4222c6f-608d-4c37-9b3f-95d625174272"),
    sig: FunSig::Fixed { tpe: Type::Unit, arg_types: vec![] },
};

pub(crate) const PRE_DEF_FUNS: [PreDefFun; 1] = [EXIT];
