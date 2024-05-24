use jati::error::Error;
use jati::symbols::fun::FunTag;
use jati::symbols::id::Id;
use jati::symbols::symbol_table::SymbolTable;
use jati::symbols::var::VarTag;
use jati::trees::types::Type;

pub(crate) struct Symbols {}

impl Symbols {
    pub(crate) fn new() -> Self {
        Symbols {}
    }
}

impl SymbolTable for Symbols {
    fn get_var(&mut self, id: &Id) -> Result<Option<VarTag>, Error> {
        Ok(None)
    }

    fn get_fun(&mut self, id: &Id, kid_types: &[Type]) -> Result<Option<FunTag>, Error> {
        Ok(None)
    }
}