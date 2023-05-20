use anyhow::{
    bail,
    Result,
};
use llvm_sys::prelude::LLVMValueRef;
use crate::{
    data::ast::{
        FnDefAst,
        LeftDefAst,
        ExprAst,
        FnAst,
        PrefixOpAst,
        InfixOpAst,
        IdentAst,
        NumAst,
    },
    llvm::LLVM,
};

pub fn compile(llvm: &mut LLVM, fn_def_asts: &Vec<FnDefAst>) -> Result<()> {
    for fn_def_ast in fn_def_asts {
        gen_fn_def(llvm, fn_def_ast)?;
    }
    Ok(())
}

fn gen_fn_def(llvm: &mut LLVM, fn_def_ast: &FnDefAst) -> Result<LLVMValueRef> {
    let f = match llvm.get_named_function(&fn_def_ast.left_def.ident.name) {
        Ok(f) => f,
        Err(_) => gen_left_def(llvm, &fn_def_ast.left_def)?,
    };
    let block_count = LLVM::count_basic_blocks(f);
    if block_count != 0 {
        bail!("Function cannot be redefined.");
    }
    let block = llvm.append_basic_block(f, "entry")?;
    llvm.position_builder_at_end(block);
    llvm.clear_named_value();
    let arg_count = fn_def_ast.left_def.args.len();
    for i in 0..arg_count {
        let arg = LLVM::get_param(f, i)?;
        llvm.set_value_name(arg, &fn_def_ast.left_def.args[i].name);
        llvm.insert_named_value(fn_def_ast.left_def.args[i].name.clone(), arg)?;
    }
    match gen_expr(llvm, &fn_def_ast.expr) {
        Ok(body) => {
            llvm.build_ret(body)?;
            if LLVM::verify_function(f) {
                Ok(f)
            }
            else {
                LLVM::delete_function(f);
                bail!("Invalid function.");
            }
        },
        Err(e) => {
            LLVM::delete_function(f);
            Err(e)
        },
    }
}

fn gen_left_def(llvm: &mut LLVM, left_def_ast: &LeftDefAst) -> Result<LLVMValueRef> {
    let arg_count = left_def_ast.args.len();
    let int_ty = llvm.int32_type()?;
    let arg_tys = vec![int_ty; arg_count];
    let f_ty = llvm.function_type(int_ty, arg_tys)?;
    let f = llvm.add_function(&left_def_ast.ident.name, f_ty)?;
    if f.is_null() {
        bail!("Cannot create function.");
    }
    Ok(f)
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
            let f = llvm.get_named_function(name)?;
            if f.is_null() {
                bail!("Unknown function");
            }
            let arg_count = LLVM::count_params(f);
            if arg_count != 1 {
                bail!("Invalid count of arguments.");
            }
            let args = vec![gen_expr(llvm, fn_ast.arg_expr.as_ref())?];
            let ty = LLVM::get_called_function_type(f)?;
            Ok(llvm.build_call(ty, f, args, "calltmp")?)
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
    llvm.get_named_value(&ident_ast.name)
}

fn gen_num(llvm: &mut LLVM, num_ast: &NumAst) -> Result<LLVMValueRef> {
    let value = num_ast.value.parse().unwrap();
    Ok(llvm.const_int(value, 0)?)
}
