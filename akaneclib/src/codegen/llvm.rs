use anyhow::{
    bail,
    Result,
};
use llvm_sys::prelude::*;
use crate::data::*;

pub fn gen_init(llvm: &mut LLVM, ctx: &SemContext) -> Result<()> {
    decl_externals(llvm)?;
    gen_add_def(llvm, ctx)?;
    Ok(())
}

pub fn gen_fn_def(llvm: &mut LLVM, ctx: &SemContext, fn_key: &FnKey, arg_names: &[String], body: impl FnOnce(&mut LLVM) -> Result<LLVMValueRef>) -> Result<()> {
    let fs = ctx.ranked_fn_store.get(&fn_key.without_rank())?.clone();
    let f = fs.last().unwrap().clone();
    let fn_value = add_fn(llvm, &f.logical_name())?;
    for i in (0..fs.len() - 1).rev() {
        let f = fs[i].clone();
        let fn_value = add_fn(llvm, &f.logical_name())?;
        let next_f = fs[i + 1].clone();
        let next_fn_value = llvm.get_named_function(&next_f.logical_name())?;
        gen_block(llvm, fn_value,
            |llvm| {
                llvm.clear_named_value();
                let thunk = LLVM::get_param(fn_value, 0)?;
                let arg_thunk = LLVM::get_param(fn_value, 1)?;
                let next_thunk = call_new_next_fn_thunk(llvm, thunk, next_fn_value, arg_thunk)?;
                llvm.build_ret(next_thunk)?;
                Ok(())
            }
        )?;
    }
    gen_block(llvm, fn_value,
        |llvm| {
            llvm.clear_named_value();
            let thunk = LLVM::get_param(fn_value, 0)?;
            for i in 0..f.arity {
                let arg = llvm.build_thunk_arg_gep(thunk, i, "gep")?;
                llvm.set_value_name(arg, &arg_names[i]);
                llvm.insert_named_value(arg_names[i].to_owned(), arg)?;
            }
            let value = body(llvm)?;
            llvm.build_ret(value)?;
            Ok(())
        }
    )
}

pub fn gen_exported_fn(llvm: &mut LLVM, ctx: &SemContext, fn_key: &FnKey, arg_names: &[String]) -> Result<()> {
    let fs = ctx.ranked_fn_store.get(&fn_key.without_rank())?.clone();
    let f = fs.first().unwrap().clone();
    let fn_value = llvm.get_named_function(&f.logical_name())?;
    let exported_fn_value = add_exported_fn(llvm, &fn_key.without_rank().logical_name(), f.arity)?;
    gen_block(llvm, exported_fn_value,
        |llvm| {
            llvm.clear_named_value();
            for i in 0..f.arity {
                let arg = LLVM::get_param(exported_fn_value, i)?;
                llvm.set_value_name(arg, &arg_names[i]);
                llvm.insert_named_value(arg_names[i].to_owned(), arg)?;
            }
            let mut arg_thunks = Vec::new();
            for i in 0..f.arity {
                let arg = LLVM::get_param(exported_fn_value, i)?;
                let thunk = call_new_val_thunk(llvm, arg)?;
                arg_thunks.push(thunk);
            }
            let null = llvm.const_null()?;
            arg_thunks.push(null);
            let arity_value = llvm.const_i64(f.arity as i64)?;
            let mut thunk = call_new_fn_thunk(llvm, fn_value, arity_value)?;
            for arg_thunk in arg_thunks {
                thunk = call_call_thunk(llvm, thunk, arg_thunk)?;
            }
            let value = eval_thunk_as_i64(llvm, thunk)?;
            llvm.build_ret(value)?;
            Ok(())
        }
    )
}

fn gen_block(llvm: &mut LLVM, fn_value: LLVMValueRef, body: impl FnOnce(&mut LLVM) -> Result<()>) -> Result<()> {
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

fn decl_externals(llvm: &mut LLVM) -> Result<()> {
    let ty = new_fn_thunk_ty(llvm)?;
    llvm.add_function("__new_fn_thunk", ty)?;
    let ty = new_next_fn_thunk_ty(llvm)?;
    llvm.add_function("__new_next_fn_thunk", ty)?;
    let ty = new_val_thunk_ty(llvm)?;
    llvm.add_function("__new_val_thunk", ty)?;
    let ty = call_thunk_ty(llvm)?;
    llvm.add_function("__call_thunk", ty)?;
    let ty = debug_print_ty(llvm)?;
    llvm.add_function("__debug_print", ty)?;
    Ok(())
}

fn new_fn_thunk_ty(llvm: &mut LLVM) -> Result<LLVMTypeRef> {
    let ptr_ty = llvm.pointer_type()?;
    let i64_ty = llvm.i64_type()?;
    llvm.function_type(ptr_ty, &[ptr_ty, i64_ty])
}

fn new_next_fn_thunk_ty(llvm: &mut LLVM) -> Result<LLVMTypeRef> {
    let ptr_ty = llvm.pointer_type()?;
    llvm.function_type(ptr_ty, &[ptr_ty, ptr_ty, ptr_ty])
}

fn new_val_thunk_ty(llvm: &mut LLVM) -> Result<LLVMTypeRef> {
    let ptr_ty = llvm.pointer_type()?;
    let i64_ty = llvm.i64_type()?;
    llvm.function_type(ptr_ty, &[i64_ty])
}

fn call_thunk_ty(llvm: &mut LLVM) -> Result<LLVMTypeRef> {
    let ptr_ty = llvm.pointer_type()?;
    llvm.function_type(ptr_ty, &[ptr_ty, ptr_ty])
}

fn debug_print_ty(llvm: &mut LLVM) -> Result<LLVMTypeRef> {
    let void_ty = LLVM::void_type()?;
    let ptr_ty = llvm.pointer_type()?;
    llvm.function_type(void_ty, &[ptr_ty])
}

fn fn_ty(llvm: &mut LLVM) -> Result<LLVMTypeRef> {
    let ptr_ty = llvm.pointer_type()?;
    llvm.function_type(ptr_ty, &[ptr_ty, ptr_ty])
}

fn exported_fn_ty(llvm: &mut LLVM, arity: usize) -> Result<LLVMTypeRef> {
    let i64_ty = llvm.i64_type()?;
    llvm.function_type(i64_ty, &vec![i64_ty; arity])
}

fn add_fn(llvm: &mut LLVM, name: &str) -> Result<LLVMValueRef> {
    let ty = fn_ty(llvm)?;
    llvm.add_function(name, ty)
}

fn add_exported_fn(llvm: &mut LLVM, name: &str, arity: usize) -> Result<LLVMValueRef> {
    let ty = exported_fn_ty(llvm, arity)?;
    llvm.add_function(name, ty)
}

pub fn call_new_fn_thunk(llvm: &mut LLVM, fn_ptr: LLVMValueRef, arity: LLVMValueRef) -> Result<LLVMValueRef> {
    let f = llvm.get_named_function("__new_fn_thunk")?;
    let ty = new_fn_thunk_ty(llvm)?;
    llvm.build_call(ty, f, &[fn_ptr, arity], "call")
}

pub fn call_new_next_fn_thunk(llvm: &mut LLVM, thunk: LLVMValueRef, fn_ptr: LLVMValueRef, arg: LLVMValueRef) -> Result<LLVMValueRef> {
    let f = llvm.get_named_function("__new_next_fn_thunk")?;
    let ty = new_next_fn_thunk_ty(llvm)?;
    llvm.build_call(ty, f, &[thunk, fn_ptr, arg], "call")
}

pub fn call_new_val_thunk(llvm: &mut LLVM, val: LLVMValueRef) -> Result<LLVMValueRef> {
    let f = llvm.get_named_function("__new_val_thunk")?;
    let ty = new_val_thunk_ty(llvm)?;
    llvm.build_call(ty, f, &[val], "call")
}

pub fn call_call_thunk(llvm: &mut LLVM, thunk: LLVMValueRef, arg: LLVMValueRef) -> Result<LLVMValueRef> {
    let f = llvm.get_named_function("__call_thunk")?;
    let ty = call_thunk_ty(llvm)?;
    llvm.build_call(ty, f, &[thunk, arg], "call")
}

pub fn _call_debug_print(llvm: &mut LLVM, thunk: LLVMValueRef) -> Result<LLVMValueRef> {
    let f = llvm.get_named_function("__debug_print")?;
    let ty = debug_print_ty(llvm)?;
    llvm.build_call(ty, f, &[thunk], "")
}

pub fn eval_thunk_as_i64(llvm: &mut LLVM, thunk: LLVMValueRef) -> Result<LLVMValueRef> {
    let thunk_ty = llvm.thunk_type()?;
    let i64_ty = llvm.i64_type()?;
    let gep = llvm.build_struct_gep(thunk_ty, thunk, 2, "gep")?;
    llvm.build_load(i64_ty, gep, "load")
}

fn gen_add_def(llvm: &mut LLVM, ctx: &SemContext) -> Result<()> {
    let fn_key = FnKey::new(QualKey::top(), "add".to_owned());
    let args = ["add.lhs".to_owned(), "add.rhs".to_owned()];
    gen_fn_def(llvm, ctx, &fn_key, &args, |llvm| {
        let lhs_thunk = llvm.get_named_value("add.lhs")?;
        let lhs = eval_thunk_as_i64(llvm, lhs_thunk)?;
        let rhs_thunk = llvm.get_named_value("add.rhs")?;
        let rhs = eval_thunk_as_i64(llvm, rhs_thunk)?;
        let add = llvm.build_add(lhs, rhs, "add")?;
        call_new_val_thunk(llvm, add)
    })
}
