use anyhow::{
    bail,
    Result,
};
use crate::data::{
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
    },
    semantics::{
        SemKey,
        scope_sem::ScopeSem,
        ty_sem::TySem,
        fn_sem::FnSem, SemVal,
    },
    thunk::Thunk,
    context::SemContext,
};

pub fn semantize(ctx: &mut SemContext, top_def_enums: &[TopDefEnum]) -> Result<()> {
    for top_def_enum in top_def_enums {
        visit_top_def(ctx, top_def_enum)?;
    }
    Ok(())
}

fn visit_top_def(ctx: &mut SemContext, top_def_enum: &TopDefEnum) -> Result<()> {
    match top_def_enum {
        TopDefEnum::Fn(fn_def_ast) => visit_fn_def(ctx, fn_def_ast)?
    }
    Ok(())
}

fn visit_fn_def(ctx: &mut SemContext, fn_def_ast: &FnDefAst) -> Result<()> {
    let qual = ctx.qual_stack.peek().get_val(ctx)?;
    let int_ty = TySem::get_from_name(ctx, "int")?;
    let fn_in_tys = vec![int_ty.clone(); fn_def_ast.left_fn_def.args.len()];
    let fn_out_ty = int_ty.clone();
    let fn_ty = TySem::new_or_get_fn_ty(ctx, qual.clone(), fn_in_tys, fn_out_ty)?;
    let fn_res = FnSem::new(ctx, qual, fn_def_ast.left_fn_def.name.clone(), fn_ty);
    match fn_res {
        Ok(f) => *fn_def_ast.fn_sem.borrow_mut() = Some(f.clone()),
        Err(_) => bail!("Duplicate function definitions."),
    }
    let qual = ctx.push_scope_into_qual_stack(ScopeSem::Fn(fn_def_ast.left_fn_def.name.clone())).get_val(ctx)?;
    visit_left_fn_def(ctx, &fn_def_ast.left_fn_def)?;
    let arg_in_tys = Vec::new();
    let arg_out_ty = int_ty;
    let arg_ty = TySem::new_or_get_fn_ty(ctx, qual.clone(), arg_in_tys, arg_out_ty)?;
    let args =
        fn_def_ast.left_fn_def.args.iter()
        .map(|arg| FnSem::new(ctx, qual.clone(), arg.clone(), arg_ty.clone()))
        .collect::<Result<_>>()?;
    *fn_def_ast.arg_sems.borrow_mut() = Some(args);
    visit_expr(ctx, &fn_def_ast.expr)?;
    ctx.qual_stack.pop()?.get_val(ctx)?;
    Ok(())
}

fn visit_left_fn_def(_ctx: &mut SemContext, _left_fn_def_ast: &LeftFnDefAst) -> Result<()> {
    Ok(())
}

fn visit_expr(ctx: &mut SemContext, expr_ast: &ExprAst) -> Result<()> {
    match &expr_ast.expr_enum {
        ExprEnum::Fn(fn_ast) => visit_fn(ctx, fn_ast)?,
        ExprEnum::PrefixOp(prefix_op_ast) => visit_prefix_op(ctx, prefix_op_ast)?,
        ExprEnum::InfixOp(infix_op_ast) => visit_infix_op(ctx, infix_op_ast)?,
        ExprEnum::Ident(ident_ast) => visit_ident(ctx, ident_ast)?,
        ExprEnum::Num(num_ast) => visit_num(ctx, num_ast)?,
    }
    let int_ty = TySem::get_from_name(ctx, "int")?;
    *expr_ast.ty_sem.borrow_mut() = Some(int_ty);
    Ok(())
}

fn visit_fn(ctx: &mut SemContext, fn_ast: &FnAst) -> Result<()> {
    visit_expr(ctx, &fn_ast.fn_expr)?;
    visit_expr(ctx, &fn_ast.arg_expr)?;
    let thunk = match &fn_ast.fn_expr.expr_enum {
        ExprEnum::Fn(prev_fn_ast) => {
            let thunk_ref = prev_fn_ast.thunk.borrow();
            let thunk = thunk_ref.as_ref().unwrap();
            let mut args = thunk.args.clone();
            args.push(fn_ast.arg_expr.clone());
            Thunk::new(thunk.fn_sem.clone(), args)
        },
        ExprEnum::Ident(ident_ast) => {
            let thunk_ref = ident_ast.thunk.borrow();
            let thunk = thunk_ref.as_ref().unwrap();
            Thunk::new(thunk.fn_sem.clone(), vec![fn_ast.arg_expr.clone()])
        },
        _ => bail!("Unsupported function evaluation."),
    };
    *fn_ast.thunk.borrow_mut() = Some(thunk);
    Ok(())
}

fn visit_prefix_op(ctx: &mut SemContext, prefix_op_ast: &PrefixOpAst) -> Result<()> {
    visit_expr(ctx, &prefix_op_ast.rhs)?;
    Ok(())
}

fn visit_infix_op(ctx: &mut SemContext, infix_op_ast: &InfixOpAst) -> Result<()> {
    visit_expr(ctx, &infix_op_ast.lhs)?;
    visit_expr(ctx, &infix_op_ast.rhs)?;
    Ok(())
}

fn visit_ident(ctx: &mut SemContext, ident_ast: &IdentAst) -> Result<()> {
    if let Some(f) = ctx.find_with_qual(
        |ctx, qual| FnSem::get(ctx, qual.to_key(), ident_ast.name.clone()).ok()
    )
    {
        *ident_ast.thunk.borrow_mut() = Some(Thunk::new(f, Vec::new()));
    }
    else {
        bail!("Unknown function.");
    }
    Ok(())
}

fn visit_num(_ctx: &mut SemContext, _num_ast: &NumAst) -> Result<()> {
    Ok(())
}
