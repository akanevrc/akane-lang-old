mod llvm;

use std::rc::Rc;
use anyhow::{
    bail,
    Result,
};
use llvm_sys::prelude::*;
use crate::data::*;

pub fn compile(llvm: &mut LLVM, ctx: &SemContext, top_def_enums: &[TopDefEnum]) -> Result<()> {
    llvm::gen_init(llvm, ctx)?;
    for top_def_enum in top_def_enums {
        gen_top_def(llvm, ctx, top_def_enum)?;
    }
    Ok(())
}

fn gen_top_def(llvm: &mut LLVM, ctx: &SemContext, top_def_enum: &TopDefEnum) -> Result<()> {
    match top_def_enum {
        TopDefEnum::Fn(fn_def_ast) =>
            gen_fn_def(llvm, ctx, fn_def_ast),
    }
}

fn gen_fn_def(llvm: &mut LLVM, ctx: &SemContext, fn_def_ast: &FnDefAst) -> Result<()> {
    let fn_key = HasRefCell::<FnKey>::get_rc(fn_def_ast);
    let args =
        HasRefCell::<Vec<Rc<FnSem>>>::get_rc(fn_def_ast).iter()
        .map(|arg| arg.logical_name())
        .collect::<Vec<_>>();
    llvm::gen_fn_def(llvm, ctx, &fn_key, &args, |llvm, _| gen_expr(llvm, &fn_def_ast.expr))?;
    llvm::gen_exported_fn(llvm, ctx, &fn_key, &args)?;
    Ok(())
}

fn gen_expr(llvm: &mut LLVM, expr_ast: &ExprAst) -> Result<LLVMValueRef> {
    match &expr_ast.expr_enum {
        ExprEnum::Fn(fn_ast) =>
            gen_fn(llvm, fn_ast),
        ExprEnum::Ident(ident_ast) =>
            gen_ident(llvm, ident_ast),
    }
}

fn gen_fn(llvm: &mut LLVM, fn_ast: &FnAst) -> Result<LLVMValueRef> {
    let fn_thunk = gen_expr(llvm, &fn_ast.fn_expr)?;
    let arg_thunk = gen_expr(llvm, &fn_ast.arg_expr)?;
    let thunk = llvm::call_call_thunk(llvm, fn_thunk, arg_thunk)?;
    let f = HasRefCell::<FnSem>::get_rc(fn_ast);
    if f.rank() == f.arity {
        let null = llvm.const_null()?;
        llvm::call_call_thunk(llvm, thunk, null)
    }
    else {
        Ok(thunk)
    }
}

fn gen_ident(llvm: &mut LLVM, ident_ast: &IdentAst) -> Result<LLVMValueRef> {
    let f = HasRefCell::<FnSem>::get_rc(ident_ast);
    let name = &f.logical_name();
    let name_without_rank = &f.to_key().without_rank().logical_name();
    if let Ok(num) = name_without_rank.parse::<i64>() {
        let value = llvm.const_i64(num)?;
        llvm::call_new_val_thunk(llvm, value)
    }
    else if let Ok(value) = llvm.get_named_value(name) {
        Ok(value)
    }
    else if let Ok(value) = llvm.get_named_function(name) {
        let arity = f.ty.arity();
        let arity_value = llvm.const_i64(arity as i64)?;
        let thunk = llvm::call_new_fn_thunk(llvm, value, arity_value)?;
        if arity == 0 {
            let null = llvm.const_null()?;
            llvm::call_call_thunk(llvm, thunk, null)
        }
        else {
            Ok(thunk)
        }
    }
    else {
        bail!("Unknown identifier.");
    }
}
