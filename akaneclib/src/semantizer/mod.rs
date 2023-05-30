use std::rc::Rc;
use anyhow::{
    bail,
    Result,
};
use crate::data::{
    ast::{
        TopDefEnum,
        FnDefAst,
        TyExprAst,
        TyExprEnum,
        TyArrowAst,
        TyIdentAst,
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
        SemVal,
        SemKey,
        scope_sem::ScopeSem,
        ty_sem::TySem,
        fn_sem::FnSem,
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
    let name = &fn_def_ast.left_fn_def.name;
    let arg_names = &fn_def_ast.left_fn_def.args;
    let fn_ty =
        if let Some(ty_annot) = &fn_def_ast.ty_annot {
            visit_ty_expr(ctx, ty_annot)?;
            ty_annot.get_rc()
        }
        else {
            let i32_ty = TySem::get_from_name(ctx, "i32")?;
            let fn_in_tys = vec![i32_ty.clone(); arg_names.len()];
            let fn_out_ty = i32_ty.clone();
            TySem::new_or_get_fn_ty(ctx, qual.clone(), fn_in_tys, fn_out_ty)?
        };
    let fn_res = FnSem::new(ctx, qual, name.clone(), fn_ty.clone());
    match fn_res {
        Ok(f) => fn_def_ast.set_rc(f.clone()),
        Err(_) => bail!("Duplicate function definitions."),
    }
    let qual = ctx.push_scope_into_qual_stack(ScopeSem::Fn(name.clone())).get_val(ctx)?;
    visit_left_fn_def(ctx, &fn_def_ast.left_fn_def)?;
    let (arg_tys, ret_ty) = fn_ty.to_arg_and_ret_tys();
    if arg_tys.len() != arg_names.len() {
        bail!("Defferent argument count between type annotation and function definition.")
    }
    let args =
        arg_names.iter()
        .zip(arg_tys)
        .map(|(name, arg_ty)| FnSem::new(ctx, qual.clone(), name.clone(), arg_ty.clone()))
        .collect::<Result<Vec<_>>>()?;
    fn_def_ast.set_rc(Rc::new(args));
    visit_expr(ctx, &fn_def_ast.expr)?;
    if ret_ty != fn_def_ast.expr.get_rc() {
        bail!("Defferent type between type annotation and function body.")
    }
    ctx.qual_stack.pop()?.get_val(ctx)?;
    Ok(())
}

fn visit_ty_expr(ctx: &mut SemContext, ty_expr_ast: &TyExprAst) -> Result<()> {
    match &ty_expr_ast.expr_enum {
        TyExprEnum::Arrow(ty_arrow) =>
            visit_ty_arrow(ctx, ty_arrow)?,
        TyExprEnum::Ident(ty_ident) =>
            visit_ty_ident(ctx, ty_ident)?,
    }
    ty_expr_ast.set_rc(ty_expr_ast.expr_enum.get_rc());
    Ok(())
}

fn visit_ty_arrow(ctx: &mut SemContext, ty_arrow_ast: &TyArrowAst) -> Result<()> {
    visit_ty_expr(ctx, &ty_arrow_ast.lhs)?;
    visit_ty_expr(ctx, &ty_arrow_ast.rhs)?;
    let qual = ctx.qual_stack.peek().get_val(ctx)?;
    let in_ty = ty_arrow_ast.lhs.get_rc();
    let out_ty = ty_arrow_ast.rhs.get_rc();
    ty_arrow_ast.set_rc(TySem::new_or_get_ty2(ctx, qual, in_ty, out_ty));
    Ok(())
}

fn visit_ty_ident(ctx: &mut SemContext, ty_ident_ast: &TyIdentAst) -> Result<()> {
    let ty_opt = ctx.find_with_qual(
        |ctx, qual| TySem::get(ctx, qual.to_key(), ty_ident_ast.name.clone()).ok()
    );
    if let Some(ty) = ty_opt {
        ty_ident_ast.set_rc(ty);
    }
    else {
        bail!("Unknown function.");
    }
    Ok(())
}

fn visit_left_fn_def(_ctx: &mut SemContext, _left_fn_def_ast: &LeftFnDefAst) -> Result<()> {
    Ok(())
}

fn visit_expr(ctx: &mut SemContext, expr_ast: &ExprAst) -> Result<()> {
    match &expr_ast.expr_enum {
        ExprEnum::Fn(fn_ast) =>
            visit_fn(ctx, fn_ast)?,
        ExprEnum::PrefixOp(prefix_op_ast) =>
            visit_prefix_op(ctx, prefix_op_ast)?,
        ExprEnum::InfixOp(infix_op_ast) =>
            visit_infix_op(ctx, infix_op_ast)?,
        ExprEnum::Ident(ident_ast) =>
            visit_ident(ctx, ident_ast)?,
        ExprEnum::Num(num_ast) =>
            visit_num(ctx, num_ast)?,
    }
    HasRefCell::<TySem>::set_rc(expr_ast, expr_ast.expr_enum.get_rc());
    Ok(())
}

fn visit_fn(ctx: &mut SemContext, fn_ast: &FnAst) -> Result<()> {
    visit_expr(ctx, &fn_ast.fn_expr)?;
    visit_expr(ctx, &fn_ast.arg_expr)?;
    let (ty, thunk) = match &fn_ast.fn_expr.expr_enum {
        ExprEnum::Fn(prev_fn_ast) => {
            let prev_ty = HasRefCell::<TySem>::get_rc(prev_fn_ast);
            let ty = prev_ty.to_applied()?;
            let prev_thunk = HasRefCell::<Thunk>::get_rc(prev_fn_ast);
            let mut args = prev_thunk.args.clone();
            args.push(fn_ast.arg_expr.clone());
            let thunk = Thunk::new(prev_thunk.fn_sem.clone(), args);
            (ty, thunk)
        },
        ExprEnum::Ident(ident_ast) => {
            let prev_ty = HasRefCell::<TySem>::get_rc(ident_ast);
            let ty = prev_ty.to_applied()?;
            let prev_thunk = HasRefCell::<Thunk>::get_rc(ident_ast);
            let thunk = Thunk::new(prev_thunk.fn_sem.clone(), vec![fn_ast.arg_expr.clone()]);
            (ty, thunk)
        },
        _ => bail!("Unsupported function evaluation."),
    };
    fn_ast.set_rc(ty);
    fn_ast.set_rc(thunk);
    Ok(())
}

fn visit_prefix_op(ctx: &mut SemContext, prefix_op_ast: &PrefixOpAst) -> Result<()> {
    visit_expr(ctx, &prefix_op_ast.rhs)?;
    HasRefCell::<TySem>::set_rc(prefix_op_ast, prefix_op_ast.rhs.get_rc());
    Ok(())
}

fn visit_infix_op(ctx: &mut SemContext, infix_op_ast: &InfixOpAst) -> Result<()> {
    visit_expr(ctx, &infix_op_ast.lhs)?;
    visit_expr(ctx, &infix_op_ast.rhs)?;
    HasRefCell::<TySem>::set_rc(infix_op_ast, infix_op_ast.lhs.get_rc());
    Ok(())
}

fn visit_ident(ctx: &mut SemContext, ident_ast: &IdentAst) -> Result<()> {
    let f_opt = ctx.find_with_qual(
        |ctx, qual| FnSem::get(ctx, qual.to_key(), ident_ast.name.clone()).ok()
    );
    if let Some(f) = f_opt {
        ident_ast.set_rc(f.ty.clone());
        ident_ast.set_rc(Thunk::new(f, Vec::new()));
    }
    else {
        bail!("Unknown function.");
    }
    Ok(())
}

fn visit_num(ctx: &mut SemContext, num_ast: &NumAst) -> Result<()> {
    let qual = ctx.qual_stack.peek().get_val(ctx)?;
    let i32_ty = TySem::get_from_name(ctx, "i32")?;
    let f = FnSem::new_or_get(ctx, qual, num_ast.value.clone(), i32_ty.clone());
    num_ast.set_rc(i32_ty);
    num_ast.set_rc(Thunk::new(f, Vec::new()));
    Ok(())
}
