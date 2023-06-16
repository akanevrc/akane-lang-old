mod pool;

use std::{
    collections::HashMap,
    ffi::CString,
    fs,
    ptr,
};
use anyhow::{
    bail,
    Result,
};
use llvm_sys::{
    *,
    analysis::*,
    core::*,
    prelude::*,
};
use pool::*;

pub trait Ptr<T>: Copy {
    fn null() -> *mut T;
    fn is_null(&self) -> bool;
}

impl Ptr<LLVMBasicBlock> for LLVMBasicBlockRef {
    fn null() -> *mut LLVMBasicBlock {
        ptr::null_mut()
    }

    fn is_null(&self) -> bool {
        LLVMBasicBlockRef::is_null(*self)
    }
}

impl Ptr<LLVMType> for LLVMTypeRef {
    fn null() -> *mut LLVMType {
        ptr::null_mut()
    }

    fn is_null(&self) -> bool {
        LLVMTypeRef::is_null(*self)
    }
}

impl Ptr<LLVMValue> for LLVMValueRef {
    fn null() -> *mut LLVMValue {
        ptr::null_mut()
    }

    fn is_null(&self) -> bool {
        LLVMValueRef::is_null(*self)
    }
}

pub struct LLVM {
    context: LLVMContextRef,
    module: LLVMModuleRef,
    builder: LLVMBuilderRef,
    named_values: HashMap<String, LLVMValueRef>,
    c_str_pool: CStrPool,
    type_slice_pool: SlicePool<LLVMType, LLVMTypeRef>,
    value_slice_pool: SlicePool<LLVMValue, LLVMValueRef>,
}

impl Drop for LLVM {
    fn drop(&mut self) {
        unsafe {
            LLVMDisposeBuilder(self.builder);
            LLVMDisposeModule(self.module);
            LLVMContextDispose(self.context);
        }
    }
}

impl LLVM {
    pub fn new(module_id: &str) -> Self {
        unsafe {
            let mut c_str_pool = CStrPool::new();
            let name = c_str_pool.c_str(module_id);
            let context = LLVMContextCreate();
            let module = LLVMModuleCreateWithNameInContext(name, context);
            let builder = LLVMCreateBuilderInContext(context);
            Self {
                context,
                module,
                builder,
                named_values: HashMap::new(),
                c_str_pool,
                type_slice_pool: SlicePool::new(),
                value_slice_pool: SlicePool::new(),
            }
        }
    }

    pub fn clear_named_value(&mut self) {
        self.named_values.clear();
    }

    pub fn get_named_value(&self, name: &str) -> Result<LLVMValueRef> {
        match self.named_values.get(name) {
            Some(value) => Ok(*value),
            None => bail!("Unknown identifier."),
        }
    }

    pub fn insert_named_value(&mut self, name: String, value: LLVMValueRef) -> Result<()> {
        match self.named_values.insert(name, value) {
            Some(_) => bail!("Duplicate identifier name."),
            None => Ok(()),
        }
    }

    fn ptr_to_result<T, LLVMRef: Ptr<T>>(value: LLVMRef) -> Result<LLVMRef> {
        if value.is_null() {
            bail!("Pointer is null.")
        }
        else {
            Ok(value)
        }
    }

    pub fn void_type() -> Result<LLVMTypeRef> {
        unsafe {
            Self::ptr_to_result(
                LLVMVoidType()
            )
        }
    }

    pub fn i32_type(&self) -> Result<LLVMTypeRef> {
        unsafe {
            Self::ptr_to_result(
                LLVMInt32TypeInContext(self.context)
            )
        }
    }

    pub fn i64_type(&self) -> Result<LLVMTypeRef> {
        unsafe {
            Self::ptr_to_result(
                LLVMInt64TypeInContext(self.context)
            )
        }
    }

    pub fn function_type(&mut self, ret_ty: LLVMTypeRef, arg_tys: &[LLVMTypeRef]) -> Result<LLVMTypeRef> {
        unsafe {
            let arg_count = arg_tys.len() as u32;
            let arg_ty_vec = self.type_slice_pool.slice(&arg_tys);
            Self::ptr_to_result(
                LLVMFunctionType(ret_ty, arg_ty_vec, arg_count, 0)
            )
        }
    }

    pub fn pointer_type(&mut self) -> Result<LLVMTypeRef> {
        unsafe {
            Self::ptr_to_result(
                LLVMPointerTypeInContext(self.context, 0)
            )
        }
    }

    pub fn type_of(value: LLVMValueRef) -> Result<LLVMTypeRef> {
        unsafe {
            Self::ptr_to_result(
                LLVMTypeOf(value)
            )
        }
    }

    pub fn array_type(ty: LLVMTypeRef, len: usize) -> Result<LLVMTypeRef> {
        unsafe {
            Self::ptr_to_result(
                LLVMArrayType(ty, len as u32)
            )
        }
    }

    pub fn struct_type(&mut self, tys: &[LLVMTypeRef]) -> Result<LLVMTypeRef> {
        unsafe {
            let ty_count = tys.len() as u32;
            let ty_vec = self.type_slice_pool.slice(&tys);
            Self::ptr_to_result(
                LLVMStructTypeInContext(self.context, ty_vec, ty_count, 1)
            )
        }
    }

    pub fn thunk_type(&mut self) -> Result<LLVMTypeRef> {
        let ptr_ty = self.pointer_type()?;
        let i64_ty = self.i64_type()?;
        self.struct_type(&[ptr_ty, i64_ty, i64_ty, ptr_ty])
    }

    pub fn const_i32(&self, value: i32) -> Result<LLVMValueRef> {
        unsafe {
            let ty = self.i32_type()?;
            Self::ptr_to_result(
                LLVMConstInt(ty, value as u64, 0)
            )
        }
    }

    pub fn const_i64(&self, value: i64) -> Result<LLVMValueRef> {
        unsafe {
            let ty = self.i64_type()?;
            Self::ptr_to_result(
                LLVMConstInt(ty, value as u64, 0)
            )
        }
    }

    pub fn const_null(&mut self) -> Result<LLVMValueRef> {
        unsafe {
            let ty = self.pointer_type()?;
            Self::ptr_to_result(
                LLVMConstNull(ty)
            )
        }
    }

    pub fn const_ptr(&mut self, value: i64) -> Result<LLVMValueRef> {
        unsafe {
            let value = self.const_i64(value)?;
            let ty = self.pointer_type()?;
            Self::ptr_to_result(
                LLVMConstIntToPtr(value, ty)
            )
        }
    }

    pub fn set_value_name(&mut self, value: LLVMValueRef, name: &str) {
        unsafe {
            LLVMSetValueName2(value, self.c_str_pool.c_str(name), name.len())
        }
    }

    pub fn add_global(&mut self, ty: LLVMTypeRef, name: &str) -> Result<LLVMValueRef> {
        unsafe {
            Self::ptr_to_result(
                LLVMAddGlobal(self.module, ty, self.c_str_pool.c_str(name))
            )
        }
    }

    pub fn add_function(&mut self, name: &str, fn_ty: LLVMTypeRef) -> Result<LLVMValueRef> {
        unsafe {
            Self::ptr_to_result(
                LLVMAddFunction(self.module, self.c_str_pool.c_str(name), fn_ty)
            )
        }
    }

    pub fn get_named_function(&mut self, name: &str) -> Result<LLVMValueRef> {
        unsafe {
            Self::ptr_to_result(
                LLVMGetNamedFunction(self.module, self.c_str_pool.c_str(name))
            )
        }
    }

    pub fn delete_function(fn_value: LLVMValueRef) {
        unsafe {
            LLVMDeleteFunction(fn_value)
        }
    }

    pub fn get_param(fn_value: LLVMValueRef, index: usize) -> Result<LLVMValueRef> {
        unsafe {
            Self::ptr_to_result(
                LLVMGetParam(fn_value, index as u32)
            )
        }
    }

    pub fn append_basic_block(&mut self, fn_value: LLVMValueRef, name: &str) -> Result<LLVMBasicBlockRef> {
        unsafe {
            Self::ptr_to_result(
                LLVMAppendBasicBlock(fn_value, self.c_str_pool.c_str(name))
            )
        }
    }

    pub fn count_basic_blocks(fn_value: LLVMValueRef) -> usize {
        unsafe {
            LLVMCountBasicBlocks(fn_value) as usize
        }
    }

    pub fn position_builder_at_end(&self, block: LLVMBasicBlockRef) {
        unsafe {
            LLVMPositionBuilderAtEnd(self.builder, block)
        }
    }

    pub fn build_call(&mut self, fn_ty: LLVMTypeRef, fn_value: LLVMValueRef, args: &[LLVMValueRef], name: &str) -> Result<LLVMValueRef> {
        unsafe {
            let arg_count = args.len() as u32;
            let arg_vec = self.value_slice_pool.slice(&args);
            let name = self.c_str_pool.c_str(name);
            Self::ptr_to_result(
                LLVMBuildCall2(self.builder, fn_ty, fn_value, arg_vec, arg_count, name)
            )
        }
    }

    pub fn build_ptr_to_int(&mut self, ptr_value: LLVMValueRef, ty: LLVMTypeRef, name: &str) -> Result<LLVMValueRef> {
        unsafe {
            Self::ptr_to_result(
                LLVMBuildPtrToInt(self.builder, ptr_value, ty, self.c_str_pool.c_str(name))
            )
        }
    }

    pub fn build_neg(&mut self, rhs: LLVMValueRef, name: &str) -> Result<LLVMValueRef> {
        unsafe {
            Self::ptr_to_result(
                LLVMBuildNeg(self.builder, rhs, self.c_str_pool.c_str(name))
            )
        }
    }

    pub fn build_add(&mut self, lhs: LLVMValueRef, rhs: LLVMValueRef, name: &str) -> Result<LLVMValueRef> {
        unsafe {
            Self::ptr_to_result(
                LLVMBuildAdd(self.builder, lhs, rhs, self.c_str_pool.c_str(name))
            )
        }
    }

    pub fn build_ret_void(&self) -> Result<LLVMValueRef> {
        unsafe {
            Self::ptr_to_result(
                LLVMBuildRetVoid(self.builder)
            )
        }
    }

    pub fn build_ret(&self, ret_value: LLVMValueRef) -> Result<LLVMValueRef> {
        unsafe {
            Self::ptr_to_result(
                LLVMBuildRet(self.builder, ret_value)
            )
        }
    }

    pub fn build_alloca(&mut self, ty: LLVMTypeRef, name: &str) -> Result<LLVMValueRef> {
        unsafe {
            Self::ptr_to_result(
                LLVMBuildAlloca(self.builder, ty, self.c_str_pool.c_str(name))
            )
        }
    }

    pub fn build_array_alloca(&mut self, ty: LLVMTypeRef, len: usize, name: &str) -> Result<LLVMValueRef> {
        unsafe {
            let len_value = self.const_i64(len as i64)?;
            Self::ptr_to_result(
                LLVMBuildArrayAlloca(self.builder, ty, len_value, self.c_str_pool.c_str(name))
            )
        }
    }

    pub fn build_load(&mut self, ty: LLVMTypeRef, ptr_value: LLVMValueRef, name: &str) -> Result<LLVMValueRef> {
        unsafe {
            Self::ptr_to_result(
                LLVMBuildLoad2(self.builder, ty, ptr_value, self.c_str_pool.c_str(name))
            )
        }
    }

    pub fn build_store(&mut self, value: LLVMValueRef, ptr_value: LLVMValueRef) -> Result<LLVMValueRef> {
        unsafe {
            Self::ptr_to_result(
                LLVMBuildStore(self.builder, value, ptr_value)
            )
        }
    }

    pub fn build_array_gep(&mut self, ty: LLVMTypeRef, ptr_value: LLVMValueRef, index: usize, name: &str) -> Result<LLVMValueRef> {
        unsafe {
            let zero_value = self.const_i64(0)?;
            let index_value = self.const_i64(index as i64)?;
            let indices = [zero_value, index_value];
            let indices_vec = self.value_slice_pool.slice(&indices);
            let name = self.c_str_pool.c_str(name);
            Self::ptr_to_result(
                LLVMBuildGEP2(self.builder, ty, ptr_value, indices_vec, 2, name)
            )
        }
    }

    pub fn build_struct_gep(&mut self, ty: LLVMTypeRef, ptr_value: LLVMValueRef, index: usize, name: &str) -> Result<LLVMValueRef> {
        unsafe {
            let zero_value = self.const_i64(0)?;
            let elem_value = self.const_i32(index as i32)?;
            let indices = [zero_value, elem_value];
            let indices_vec = self.value_slice_pool.slice(&indices);
            let name = self.c_str_pool.c_str(name);
            Self::ptr_to_result(
                LLVMBuildGEP2(self.builder, ty, ptr_value, indices_vec, 2, name)
            )
        }
    }

    pub fn build_thunk_arg_gep(&mut self, ptr_value: LLVMValueRef, index: usize, name: &str) -> Result<LLVMValueRef> {
        unsafe {
            let ty = self.thunk_type()?;
            let zero_value = self.const_i64(0)?;
            let elem_value = self.const_i32(3 as i32)?;
            let indices = [zero_value, elem_value];
            let indices_vec = self.value_slice_pool.slice(&indices);
            let name = self.c_str_pool.c_str(name);
            let gep = Self::ptr_to_result(
                LLVMBuildGEP2(self.builder, ty, ptr_value, indices_vec, 2, name)
            )?;
            let ptr_ty = self.pointer_type()?;
            let arg_value = self.build_load(ptr_ty, gep, "load")?;
            let index_value = self.const_i64(index as i64)?;
            let indices = [index_value];
            let indices_vec = self.value_slice_pool.slice(&indices);
            let gep = Self::ptr_to_result(
                LLVMBuildGEP2(self.builder, ptr_ty, arg_value, indices_vec, 1, name)
            )?;
            self.build_load(ptr_ty, gep, "load")
        }
    }

    pub fn verify_function(fn_value: LLVMValueRef) -> bool {
        unsafe {
            LLVMVerifyFunction(fn_value, LLVMVerifierFailureAction::LLVMPrintMessageAction) == 0
        }
    }

    pub fn print_module_to_file(&mut self, path: &str) -> Result<()> {
        unsafe {
            let message = CString::from_raw(LLVMPrintModuleToString(self.module));
            fs::write(path, message.to_bytes())?;
            LLVMDisposeMessage(message.into_raw());
            Ok(())
        }
    }
}
