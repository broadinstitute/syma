use jati::trees::typed::tree::Tree;
use crate::interpreter::RunState;

pub(crate) struct Executor {}

impl Executor {
    pub(crate) fn new() -> Self { Executor {} }
    pub(crate) fn execute(&self, tree: Tree, state: &mut RunState) {

    }
}