use std::rc::Rc;
use anyhow::{
    bail,
    Result,
};
use llvm_sys::prelude::LLVMValueRef;
use crate::{
    data::{
        ast::{
            TopDefEnum,
            FnDefAst,
            LeftFnDefAst,
            ExprAst,
            ExprEnum,
            FnAst,
            PrefixOpAst,
            InfixOpAst,
            IdentAst,
            NumAst,
            HasRefCell,
        },
        semantics::{
            Sem,
            fn_sem::FnSem,
        },
        thunk::Thunk,
        llvm::LLVM,
    },
};

pub fn compile(llvm: &mut LLVM, top_def_enums: &[TopDefEnum]) -> Result<()> {
    for top_def_enum in top_def_enums {
        gen_top_def(llvm, top_def_enum)?;
    }
    Ok(())
}

fn gen_top_def(llvm: &mut LLVM, top_def_enum: &TopDefEnum) -> Result<LLVMValueRef> {
    match top_def_enum {
        TopDefEnum::Fn(fn_def_ast) =>
            gen_fn_def(llvm, fn_def_ast),
    }
}

fn gen_fn_def(llvm: &mut LLVM, fn_def_ast: &FnDefAst) -> Result<LLVMValueRef> {
    let fn_value = match llvm.get_named_function(&fn_def_ast.left_fn_def.name) {
        Ok(f) => f,
        Err(_) => gen_left_fn_def(llvm, &fn_def_ast.left_fn_def)?,
    };
    llvm.clear_named_value();
    let rank = fn_def_ast.left_fn_def.args.len();
    let args = HasRefCell::<Vec<Rc<FnSem>>>::get_rc(fn_def_ast);
    for i in 0..rank {
        let arg = LLVM::get_param(fn_value, i)?;
        let name = &args[i].logical_name();
        llvm.set_value_name(arg, name);
        llvm.insert_named_value(name.to_owned(), arg)?;
    }
    gen_llvm_block(llvm, fn_value,
        |llvm| {
            match gen_expr(llvm, &fn_def_ast.expr)? {
                Some(expr) => llvm.build_ret(expr)?,
                None => llvm.build_ret_void()?,
            };
            Ok(())
        }
    )?;
    Ok(fn_value)
}

fn gen_left_fn_def(llvm: &mut LLVM, left_fn_def_ast: &LeftFnDefAst) -> Result<LLVMValueRef> {
    let rank = left_fn_def_ast.args.len();
    let i32_ty = llvm.int32_type()?;
    let arg_tys = vec![i32_ty; rank];
    let fn_ty = llvm.function_type(i32_ty, arg_tys)?;
    let fn_value_res = llvm.add_function(&left_fn_def_ast.name, fn_ty);
    match fn_value_res {
        Ok(f) => Ok(f),
        Err(_) => bail!("Cannot create function."),
    }
}

fn gen_expr(llvm: &mut LLVM, expr_ast: &ExprAst) -> Result<Option<LLVMValueRef>> {
    match &expr_ast.expr_enum {
        ExprEnum::Fn(fn_ast) =>
            gen_fn(llvm, fn_ast),
        ExprEnum::PrefixOp(prefix_op_ast) =>
            gen_prefix_op(llvm, prefix_op_ast),
        ExprEnum::InfixOp(infix_op_ast) =>
            gen_infix_op(llvm, infix_op_ast),
        ExprEnum::Ident(ident_ast) =>
            gen_ident(llvm, ident_ast),
        ExprEnum::Num(num_ast) =>
            gen_num(llvm, num_ast),
    }?;
    let thunk = HasRefCell::<Thunk>::get_rc(expr_ast);
    if thunk.is_callable() {
        match thunk.fn_sem.logical_name().as_str() {
            "+" => {
                let lhs = gen_expr(llvm, &thunk.args[0])?.unwrap();
                let rhs = gen_expr(llvm, &thunk.args[1])?.unwrap();
                Ok(Some(llvm.build_add(lhs, rhs, "addtmp")?))
            },
            name => {
                if let Ok(value) = name.parse() {
                    Ok(Some(llvm.const_int(value, 0)?))
                }
                else if let Ok(value) = llvm.get_named_value(name) {
                    Ok(Some(value))
                }
                else if llvm.get_named_function(name).is_ok() {
                    let fn_value = llvm.get_named_function(&thunk.fn_sem.logical_name())?;
                    let i32_ty = llvm.int32_type()?;
                    let arg_tys = vec![i32_ty; thunk.fn_sem.rank];
                    let fn_ty = llvm.function_type(i32_ty, arg_tys)?;
                    let args =
                        thunk.args.iter()
                        .cloned()
                        .map(|arg| Ok(gen_expr(llvm, arg.as_ref())?.unwrap()))
                        .collect::<Result<_>>()?;
                    Ok(Some(llvm.build_call(fn_ty, fn_value, args, "calltmp")?))
                }
                else {
                    bail!("Unknown identifier.")
                }
            },
        }
    }
    else {
        Ok(None)
    }
}

fn gen_fn(llvm: &mut LLVM, fn_ast: &FnAst) -> Result<Option<LLVMValueRef>> {
    gen_expr(llvm, &fn_ast.fn_expr)
}

fn gen_prefix_op(_llvm: &mut LLVM, _prefix_op_ast: &PrefixOpAst) -> Result<Option<LLVMValueRef>> {
    panic!("Not implemented.")
}

fn gen_infix_op(_llvm: &mut LLVM, _infix_op_ast: &InfixOpAst) -> Result<Option<LLVMValueRef>> {
    Ok(None)
}

fn gen_ident(_llvm: &mut LLVM, _ident_ast: &IdentAst) -> Result<Option<LLVMValueRef>> {
    Ok(None)
}

fn gen_num(_llvm: &mut LLVM, _num_ast: &NumAst) -> Result<Option<LLVMValueRef>> {
    Ok(None)
}

fn gen_llvm_block(llvm: &mut LLVM, fn_value: LLVMValueRef, body: impl FnOnce(&mut LLVM) -> Result<()>) -> Result<()> {
    let block_count = LLVM::count_basic_blocks(fn_value);
    if block_count != 0 {
        bail!("Function cannot be redefined.");
    }
    let block = llvm.append_basic_block(fn_value, "entry")?;
    llvm.position_builder_at_end(block);
    match body(llvm) {
        Ok(_) => {
            if LLVM::verify_function(fn_value) {
                Ok(())
            }
            else {
                LLVM::delete_function(fn_value);
                bail!("Invalid function.");
            }
        },
        Err(e) => {
            LLVM::delete_function(fn_value);
            Err(e)
        },
    }
}
