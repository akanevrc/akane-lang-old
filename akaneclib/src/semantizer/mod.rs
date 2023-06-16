use std::rc::Rc;
use anyhow::{
    bail,
    Result,
};
use crate::data::*;

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
            let i64_ty = TySem::get_from_name(ctx, "i64")?;
            let fn_in_tys = vec![i64_ty.clone(); arg_names.len()];
            let fn_out_ty = i64_ty.clone();
            TySem::new_or_get_fn_ty(ctx, qual.clone(), fn_in_tys, fn_out_ty)
        };
    let fn_res = FnSem::new(ctx, qual, name.clone(), fn_ty.clone());
    match fn_res {
        Ok(f) =>
            fn_def_ast.set_rc(Rc::new(f[0].to_key())),
        Err(_) =>
            bail!("Duplicate function definitions."),
    };
    let qual = ctx.push_scope_into_qual_stack(ScopeSem::Fn(name.clone())).get_val(ctx)?;
    let (arg_tys, ret_ty) = fn_ty.to_arg_and_ret_tys();
    if arg_tys.len() != arg_names.len() {
        bail!("Defferent argument count between type annotation and function definition.")
    }
    let args =
        arg_names.iter()
        .zip(arg_tys)
        .map(|(name, arg_ty)| Ok(FnSem::new(ctx, qual.clone(), name.clone(), arg_ty.clone())?.first().unwrap().clone()))
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

fn visit_expr(ctx: &mut SemContext, expr_ast: &ExprAst) -> Result<()> {
    match &expr_ast.expr_enum {
        ExprEnum::Fn(fn_ast) =>
            visit_fn(ctx, fn_ast)?,
        ExprEnum::Ident(ident_ast) =>
            visit_ident(ctx, ident_ast)?,
    }
    HasRefCell::<TySem>::set_rc(expr_ast, expr_ast.expr_enum.get_rc());
    HasRefCell::<FnSem>::set_rc(expr_ast, expr_ast.expr_enum.get_rc());
    Ok(())
}

fn visit_fn(ctx: &mut SemContext, fn_ast: &FnAst) -> Result<()> {
    visit_expr(ctx, &fn_ast.fn_expr)?;
    visit_expr(ctx, &fn_ast.arg_expr)?;
    let (ty, f) = match &fn_ast.fn_expr.expr_enum {
        ExprEnum::Fn(prev_fn_ast) => {
            let prev_ty = HasRefCell::<TySem>::get_rc(prev_fn_ast);
            let ty = prev_ty.to_out_ty()?;
            let prev_f = HasRefCell::<FnSem>::get_rc(prev_fn_ast);
            let next_f = ctx.next_fn_store.get(&prev_f.to_key())?.clone();
            (ty, next_f)
        },
        ExprEnum::Ident(ident_ast) => {
            let prev_ty = HasRefCell::<TySem>::get_rc(ident_ast);
            let ty = prev_ty.to_out_ty()?;
            let prev_f = HasRefCell::<FnSem>::get_rc(ident_ast);
            let next_f = ctx.next_fn_store.get(&prev_f.to_key())?.clone();
            (ty, next_f)
        },
    };
    fn_ast.set_rc(ty);
    fn_ast.set_rc(f);
    Ok(())
}

fn visit_ident(ctx: &mut SemContext, ident_ast: &IdentAst) -> Result<()> {
    if is_num(&ident_ast.name) {
        let top = QualSem::top(ctx);
        let i64_ty = TySem::get_from_name(ctx, "i64")?;
        ident_ast.set_rc(i64_ty.clone());
        let f = FnSem::new_or_get(ctx, top, ident_ast.name.clone(), i64_ty);
        ident_ast.set_rc(f.last().unwrap().clone());
        return Ok(())
    }
    let f_opt =
        ctx.find_with_qual(|ctx, qual| {
            let key = FnKey::new(qual.to_key(), ident_ast.name.clone());
            ctx.ranked_fn_store.get(&key).ok()
            .map(|fs| fs.first().unwrap().clone())
        });
    if let Some(f) = f_opt {
        ident_ast.set_rc(f.ty.clone());
        ident_ast.set_rc(f);
        Ok(())
    }
    else {
        bail!("Unknown function.");
    }
}

fn is_num(name: &str) -> bool {
    name.chars().next().map_or(false, |c| c.is_ascii_digit())
}
