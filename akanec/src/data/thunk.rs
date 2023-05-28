use std::rc::Rc;
use super::{
    ast::ExprAst,
    semantics::fn_sem::FnSem,
};

#[derive(Clone, Debug, PartialEq)]
pub struct Thunk {
    pub fn_sem: Rc<FnSem>,
    pub args: Vec<Rc<ExprAst>>,
}

impl Thunk {
    pub fn new(fn_sem: Rc<FnSem>, args: Vec<Rc<ExprAst>>) -> Rc<Self> {
        Rc::new(Self { fn_sem, args })
    }

    pub fn is_callable(&self) -> bool {
        self.fn_sem.rank == self.args.len()
    }
}
