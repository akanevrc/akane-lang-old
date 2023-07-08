use std::rc::Rc;
use anyhow::{
    Error,
    Result,
};
use crate::data::*;
use crate::anyhow_info;

macro_rules! anyhow_ast_with_line {
    ($ast:expr, $msg:expr, $($arg:tt)*) => {
        {
            let info = &$ast.str_info;
            let target_part_of_line = format!("\n{}", info.target_part_of_line());
            anyhow_info!(info, $msg, $($arg),* target_part_of_line)
        }
    };
}

macro_rules! bail_ast_with_line {
    ($errs:ident, $ast:expr, $msg:expr, $($arg:tt)*) => {
        {
            $errs.push(anyhow_ast_with_line!($ast, $msg, $($arg),*));
            return Err($errs);
        }
    };
}

macro_rules! result_with_errors {
    ($errs:ident) => {
        if $errs.len() == 0 {
            Ok(())
        }
        else {
            Err($errs)
        }
    };
}

macro_rules! return_with_errors {
    ($errs:ident) => {
        if $errs.len() != 0 {
            return Err($errs)
        }
    };
}

macro_rules! visit_with_errors {
    ($matchee:expr, $errs:ident) => {
        match $matchee {
            Ok(_) => (),
            Err(mut es) => $errs.append(&mut es),
        }
    };
}

macro_rules! try_with_errors {
    ($matchee:expr, $ast:expr, $errs:ident) => {
        match $matchee {
            Ok(val) => val,
            Err(e) => {
                $errs.push(anyhow_ast_with_line!($ast, "{}{}", e));
                return Err($errs);
            },
        }
    };
}

pub fn semantize(ctx: &mut SemContext, top_def_enums: &[TopDefEnum]) -> Result<(), Vec<Error>> {
    let mut errs = Vec::new();
    for top_def_enum in top_def_enums {
        visit_with_errors!(visit_top_def(ctx, top_def_enum), errs);
    }
    result_with_errors!(errs)
}

fn visit_top_def(ctx: &mut SemContext, top_def_enum: &TopDefEnum) -> Result<(), Vec<Error>> {
    match top_def_enum {
        TopDefEnum::Fn(fn_def_ast) => visit_fn_def(ctx, fn_def_ast)?,
    }
    Ok(())
}

fn visit_fn_def(ctx: &mut SemContext, fn_def_ast: &FnDefAst) -> Result<(), Vec<Error>> {
    let mut errs = Vec::new();
    let qual = try_with_errors!(ctx.qual_stack.peek().get_val(ctx), fn_def_ast.left_fn_def, errs);
    let name = &fn_def_ast.left_fn_def.name;
    let arg_names = &fn_def_ast.left_fn_def.args;
    let fn_ty =
        if let Some(ty_annot) = &fn_def_ast.ty_annot {
            visit_with_errors!(visit_ty_expr(ctx, ty_annot), errs);
            return_with_errors!(errs);
            ty_annot.get_rc()
        }
        else {
            let i64_ty = try_with_errors!(TySem::get_from_name(ctx, "i64"), fn_def_ast.left_fn_def, errs);
            let fn_in_tys = vec![i64_ty.clone(); arg_names.len()];
            let fn_out_ty = i64_ty.clone();
            TySem::new_or_get_fn_ty(ctx, qual.clone(), fn_in_tys, fn_out_ty)
        };
    let fn_res = FnSem::new(ctx, qual, name.clone(), fn_ty.clone());
    match fn_res {
        Ok(f) =>
            fn_def_ast.set_rc(Rc::new(f[0].to_key())),
        Err(_) =>
            bail_ast_with_line!(errs, fn_def_ast.left_fn_def, "Duplicate function definitions: `{}`{}", name),
    };
    let qual = try_with_errors!(ctx.push_scope_into_qual_stack(ScopeSem::Fn(name.clone())).get_val(ctx), fn_def_ast.left_fn_def, errs);
    let (arg_tys, ret_ty) = fn_ty.to_arg_and_ret_tys();
    if arg_tys.len() != arg_names.len() {
        bail_ast_with_line!(errs, fn_def_ast.left_fn_def, "Defferent argument count between type annotation and function definition: `{}`{}", name);
    }
    let args =
        try_with_errors!(
            arg_names.iter()
            .zip(arg_tys)
            .map(|(name, arg_ty)| Ok(FnSem::new(ctx, qual.clone(), name.clone(), arg_ty.clone())?.first().unwrap().clone()))
            .collect::<Result<Vec<_>>>(),
            fn_def_ast.left_fn_def,
            errs
        );
    fn_def_ast.set_rc(Rc::new(args));
    visit_expr(ctx, &fn_def_ast.expr)?;
    if ret_ty != fn_def_ast.expr.get_rc() {
        bail_ast_with_line!(errs, fn_def_ast.left_fn_def, "Defferent type between type annotation and function body: `{}`{}", name);
    }
    try_with_errors!(ctx.qual_stack.pop(), fn_def_ast.left_fn_def, errs);
    Ok(())
}

fn visit_ty_expr(ctx: &mut SemContext, ty_expr_ast: &TyExprAst) -> Result<(), Vec<Error>> {
    match &ty_expr_ast.expr_enum {
        TyExprEnum::Arrow(ty_arrow) =>
            visit_ty_arrow(ctx, ty_arrow)?,
        TyExprEnum::Ident(ty_ident) =>
            visit_ty_ident(ctx, ty_ident)?,
    }
    ty_expr_ast.set_rc(ty_expr_ast.expr_enum.get_rc());
    Ok(())
}

fn visit_ty_arrow(ctx: &mut SemContext, ty_arrow_ast: &TyArrowAst) -> Result<(), Vec<Error>> {
    let mut errs = Vec::new();
    visit_with_errors!(visit_ty_expr(ctx, &ty_arrow_ast.lhs), errs);
    visit_with_errors!(visit_ty_expr(ctx, &ty_arrow_ast.rhs), errs);
    return_with_errors!(errs);
    let qual = try_with_errors!(ctx.qual_stack.peek().get_val(ctx), ty_arrow_ast, errs);
    let in_ty = ty_arrow_ast.lhs.get_rc();
    let out_ty = ty_arrow_ast.rhs.get_rc();
    ty_arrow_ast.set_rc(TySem::new_or_get_ty2(ctx, qual, in_ty, out_ty));
    result_with_errors!(errs)
}

fn visit_ty_ident(ctx: &mut SemContext, ty_ident_ast: &TyIdentAst) -> Result<(), Vec<Error>> {
    let mut errs = Vec::new();
    let ty_opt = ctx.find_with_qual(
        |ctx, qual| TySem::get(ctx, qual.to_key(), ty_ident_ast.name.clone()).ok()
    );
    if let Some(ty) = ty_opt {
        ty_ident_ast.set_rc(ty);
    }
    else {
        bail_ast_with_line!(errs, ty_ident_ast, "Unknown type: `{}`{}", (ty_ident_ast.name));
    }
    result_with_errors!(errs)
}

fn visit_expr(ctx: &mut SemContext, expr_ast: &ExprAst) -> Result<(), Vec<Error>> {
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

fn visit_fn(ctx: &mut SemContext, fn_ast: &FnAst) -> Result<(), Vec<Error>> {
    let mut errs = Vec::new();
    visit_with_errors!(visit_expr(ctx, &fn_ast.fn_expr), errs);
    visit_with_errors!(visit_expr(ctx, &fn_ast.arg_expr), errs);
    return_with_errors!(errs);
    let (ty, f) = match &fn_ast.fn_expr.expr_enum {
        ExprEnum::Fn(prev_fn_ast) => {
            let prev_ty = HasRefCell::<TySem>::get_rc(prev_fn_ast);
            let ty = try_with_errors!(prev_ty.to_out_ty(), fn_ast, errs);
            let prev_f = HasRefCell::<FnSem>::get_rc(prev_fn_ast);
            let next_f = try_with_errors!(ctx.next_fn_store.get(&prev_f.to_key()), fn_ast, errs);
            (ty, next_f.clone())
        },
        ExprEnum::Ident(ident_ast) => {
            let prev_ty = HasRefCell::<TySem>::get_rc(ident_ast);
            let ty = try_with_errors!(prev_ty.to_out_ty(), fn_ast, errs);
            let prev_f = HasRefCell::<FnSem>::get_rc(ident_ast);
            let next_f = try_with_errors!(ctx.next_fn_store.get(&prev_f.to_key()), fn_ast, errs);
            (ty, next_f.clone())
        },
    };
    fn_ast.set_rc(ty);
    fn_ast.set_rc(f);
    result_with_errors!(errs)
}

fn visit_ident(ctx: &mut SemContext, ident_ast: &IdentAst) -> Result<(), Vec<Error>> {
    match &ident_ast.ident {
        IdentEnum::Ident(ident) => {
            let f_opt =
                ctx.find_with_qual(|ctx, qual| {
                    let key = FnKey::new(qual.to_key(), ident.clone());
                    ctx.ranked_fn_store.get(&key).ok()
                    .map(|fs| fs.first().unwrap().clone())
                });
            if let Some(f) = f_opt {
                ident_ast.set_rc(f.ty.clone());
                ident_ast.set_rc(f);
                Ok(())
            }
            else {
                let mut errs = Vec::new();
                bail_ast_with_line!(errs, ident_ast, "Unknown function: `{}`{}", (ident));
            }
        },
        IdentEnum::Int(int) => {
            let mut errs = Vec::new();
            let top = QualSem::top(ctx);
            let i64_ty = try_with_errors!(TySem::get_from_name(ctx, "i64"), ident_ast, errs);
            ident_ast.set_rc(i64_ty.clone());
            let f = FnSem::new_or_get(ctx, top, int.clone(), i64_ty);
            ident_ast.set_rc(f.last().unwrap().clone());
            Ok(())
        },
        IdentEnum::Float(float) => {
            let mut errs = Vec::new();
            let top = QualSem::top(ctx);
            let f64_ty = try_with_errors!(TySem::get_from_name(ctx, "f64"), ident_ast, errs);
            ident_ast.set_rc(f64_ty.clone());
            let f = FnSem::new_or_get(ctx, top, float.clone(), f64_ty);
            ident_ast.set_rc(f.last().unwrap().clone());
            Ok(())
        },
    }
}
