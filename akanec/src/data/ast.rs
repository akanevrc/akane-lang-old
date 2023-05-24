use std::rc::Rc;

#[derive(Clone, Debug, PartialEq)]
pub enum TopDefAst {
    Fn(FnDefAst),
}

#[derive(Clone, Debug, PartialEq)]
pub struct FnDefAst {
    pub left_fn_def: LeftFnDefAst,
    pub expr: ExprAst,
}

#[derive(Clone, Debug, PartialEq)]
pub struct LeftFnDefAst {
    pub ident: IdentAst,
    pub args: Vec<IdentAst>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum ExprAst {
    Fn(FnAst),
    PrefixOp(PrefixOpAst),
    InfixOp(InfixOpAst),
    Ident(IdentAst),
    Num(NumAst),
}

#[derive(Clone, Debug, PartialEq)]
pub struct FnAst {
    pub fn_expr: Rc<ExprAst>,
    pub arg_expr: Rc<ExprAst>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct PrefixOpAst {
    pub op_code: String,
    pub rhs: Rc<ExprAst>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct InfixOpAst {
    pub op_code: String,
    pub lhs: Rc<ExprAst>,
    pub rhs: Rc<ExprAst>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct IdentAst {
    pub name: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct NumAst {
    pub value: String,
}
