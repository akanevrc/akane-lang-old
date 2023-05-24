use anyhow::{
    bail,
    Result,
};
use llvm_sys::prelude::LLVMValueRef;
use crate::{
    data::ast::{
        TopDefAst,
        FnDefAst,
        LeftFnDefAst,
        ExprAst,
        FnAst,
        PrefixOpAst,
        InfixOpAst,
        IdentAst,
        NumAst,
    },
    llvm::LLVM,
};

pub fn compile(llvm: &mut LLVM, top_def_asts: &[TopDefAst]) -> Result<()> {
    for top_def_ast in top_def_asts {
        gen_top_def(llvm, top_def_ast)?;
    }
    Ok(())
}

fn gen_top_def(llvm: &mut LLVM, top_def_ast: &TopDefAst) -> Result<LLVMValueRef> {
    match top_def_ast {
        TopDefAst::Fn(fn_def_ast) =>
            gen_fn_def(llvm, fn_def_ast),
    }
}

fn gen_fn_def(llvm: &mut LLVM, fn_def_ast: &FnDefAst) -> Result<LLVMValueRef> {
    let fn_value = match llvm.get_named_function(&fn_def_ast.left_fn_def.ident.name) {
        Ok(f) => f,
        Err(_) => gen_left_fn_def(llvm, &fn_def_ast.left_fn_def)?,
    };
    llvm.clear_named_value();
    let arg_count = fn_def_ast.left_fn_def.args.len();
    for i in 0..arg_count {
        let arg = LLVM::get_param(fn_value, i)?;
        llvm.set_value_name(arg, &fn_def_ast.left_fn_def.args[i].name);
        llvm.insert_named_value(fn_def_ast.left_fn_def.args[i].name.clone(), arg)?;
    }
    gen_llvm_block(llvm, fn_value,
        |llvm| {
            let expr = gen_expr(llvm, &fn_def_ast.expr)?;
            llvm.build_ret(expr)?;
            Ok(())
        }
    )?;
    Ok(fn_value)
}

fn gen_left_fn_def(llvm: &mut LLVM, left_fn_def_ast: &LeftFnDefAst) -> Result<LLVMValueRef> {
    let arg_count = left_fn_def_ast.args.len();
    let int_ty = llvm.int32_type()?;
    let arg_tys = vec![int_ty; arg_count];
    let fn_ty = llvm.function_type(int_ty, arg_tys)?;
    let fn_value_res = llvm.add_function(&left_fn_def_ast.ident.name, fn_ty);
    match fn_value_res {
        Ok(f) => Ok(f),
        Err(_) => bail!("Cannot create function."),
    }
}

fn gen_expr(llvm: &mut LLVM, expr_ast: &ExprAst) -> Result<LLVMValueRef> {
    match expr_ast {
        ExprAst::Fn(fn_ast) =>
            gen_fn(llvm, fn_ast),
        ExprAst::PrefixOp(prefix_op_ast) =>
            gen_prefix_op(llvm, prefix_op_ast),
        ExprAst::InfixOp(infix_op_ast) =>
            gen_infix_op(llvm, infix_op_ast),
        ExprAst::Ident(ident_ast) =>
            gen_ident(llvm, ident_ast),
        ExprAst::Num(num_ast) =>
            gen_num(llvm, num_ast),
    }
}

fn gen_fn(llvm: &mut LLVM, fn_ast: &FnAst) -> Result<LLVMValueRef> {
    match fn_ast.fn_expr.as_ref() {
        ExprAst::Ident(IdentAst { name }) => {
            let fn_value = llvm.get_named_function(name)?;
            if fn_value.is_null() {
                bail!("Unknown function");
            }
            let arg_count = LLVM::count_params(fn_value);
            if arg_count != 1 {
                bail!("Invalid count of arguments.");
            }
            let args = vec![gen_expr(llvm, fn_ast.arg_expr.as_ref())?];
            let ty = LLVM::get_called_function_type(fn_value)?;
            Ok(llvm.build_call(ty, fn_value, args, "calltmp")?)
        },
        _ => panic!("Not implemented."),
    }
}

fn gen_prefix_op(_llvm: &mut LLVM, _prefix_op_ast: &PrefixOpAst) -> Result<LLVMValueRef> {
    panic!("Not implemented.")
}

fn gen_infix_op(llvm: &mut LLVM, infix_op_ast: &InfixOpAst) -> Result<LLVMValueRef> {
    match infix_op_ast.op_code.as_str() {
        "+" => {
            let lhs = gen_expr(llvm, infix_op_ast.lhs.as_ref())?;
            let rhs = gen_expr(llvm, infix_op_ast.rhs.as_ref())?;
            Ok(llvm.build_add(lhs, rhs, "addtmp")?)
        },
        _ => bail!("Invalid operator."),
    }
}

fn gen_ident(llvm: &mut LLVM, ident_ast: &IdentAst) -> Result<LLVMValueRef> {
    if let Ok(value) = llvm.get_named_value(&ident_ast.name) {
        Ok(value)
    }
    else {
        bail!("Unknown identifier.")
    }
}

fn gen_num(llvm: &mut LLVM, num_ast: &NumAst) -> Result<LLVMValueRef> {
    let value = num_ast.value.parse().unwrap();
    Ok(llvm.const_int(value, 0)?)
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
