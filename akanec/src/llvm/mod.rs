mod pool;

use std::{
    collections::HashMap,
    ffi::CString,
    fs,
};
use anyhow::{
    anyhow,
    bail,
    Result,
};
use llvm_sys::{
    analysis::{
        LLVMVerifierFailureAction,
        LLVMVerifyFunction,
    },
    core::{
        LLVMAddFunction,
        LLVMAppendBasicBlock,
        LLVMBuildAdd,
        LLVMBuildCall2,
        LLVMBuildRet,
        LLVMConstInt,
        LLVMContextCreate,
        LLVMContextDispose,
        LLVMCountBasicBlocks,
        LLVMCountParams,
        LLVMCreateBuilderInContext,
        LLVMDeleteFunction,
        LLVMDisposeBuilder,
        LLVMDisposeMessage,
        LLVMDisposeModule,
        LLVMFunctionType,
        LLVMGetCalledFunctionType,
        LLVMGetNamedFunction,
        LLVMGetParam,
        LLVMInt32TypeInContext,
        LLVMModuleCreateWithNameInContext,
        LLVMPositionBuilderAtEnd,
        LLVMPrintModuleToString,
        LLVMSetValueName2,
    },
    prelude::{
        LLVMBasicBlockRef,
        LLVMBuilderRef,
        LLVMContextRef,
        LLVMModuleRef,
        LLVMTypeRef,
        LLVMValueRef,
    },
};
use pool::Pool;

trait Ptr: Copy {
    fn is_null(&self) -> bool;
}

impl Ptr for LLVMBasicBlockRef {
    fn is_null(&self) -> bool {
        LLVMBasicBlockRef::is_null(*self)
    }
}

impl Ptr for LLVMTypeRef {
    fn is_null(&self) -> bool {
        LLVMTypeRef::is_null(*self)
    }
}

impl Ptr for LLVMValueRef {
    fn is_null(&self) -> bool {
        LLVMValueRef::is_null(*self)
    }
}

pub struct LLVM {
    context: LLVMContextRef,
    module: LLVMModuleRef,
    builder: LLVMBuilderRef,
    named_values: HashMap<String, LLVMValueRef>,
    pool: Pool,
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
            let mut pool = Pool::new();
            let name = pool.c_str(module_id);
            let context = LLVMContextCreate();
            let module = LLVMModuleCreateWithNameInContext(name, context);
            let builder = LLVMCreateBuilderInContext(context);
            let named_values = HashMap::new();
            Self { context, module, builder, named_values, pool }
        }
    }

    pub fn clear_named_value(&mut self) {
        self.named_values.clear();
    }

    pub fn get_named_value(&self, name: &str) -> Result<LLVMValueRef> {
        self.named_values.get(name)
        .map(|value| *value)
        .ok_or(anyhow!("Unknown identifier."))
    }

    pub fn insert_named_value(&mut self, name: String, value: LLVMValueRef) -> Result<LLVMValueRef> {
        match self.named_values.insert(name, value) {
            Some(_) => bail!("Duplicate identifier name."),
            None => Ok(value),
        }
    }

    fn ptr_to_result<LLVMRef: Ptr>(value: LLVMRef) -> Result<LLVMRef> {
        if value.is_null() {
            bail!("Pointer is null.")
        }
        else {
            Ok(value)
        }
    }

    pub fn int32_type(&self) -> Result<LLVMTypeRef> {
        unsafe {
            Self::ptr_to_result(
                LLVMInt32TypeInContext(self.context)
            )
        }
    }

    pub fn function_type(&mut self, ret_ty: LLVMTypeRef, arg_tys: Vec<LLVMTypeRef>) -> Result<LLVMTypeRef> {
        unsafe {
            let arg_count = arg_tys.len() as u32;
            let arg_ty_vec = self.pool.ptr_vec(arg_tys);
            Self::ptr_to_result(
                LLVMFunctionType(ret_ty, arg_ty_vec, arg_count, 0)
            )
        }
    }

    pub fn get_called_function_type(fn_value: LLVMValueRef) -> Result<LLVMTypeRef> {
        unsafe {
            Self::ptr_to_result(
                LLVMGetCalledFunctionType(fn_value)
            )
        }
    }

    pub fn const_int(&self, value: u64, sign_extend: i32) -> Result<LLVMValueRef> {
        unsafe {
            let ty = LLVMInt32TypeInContext(self.context);
            Self::ptr_to_result(
                LLVMConstInt(ty, value, sign_extend)
            )
        }
    }

    pub fn set_value_name(&mut self, value: LLVMValueRef, name: &str) {
        unsafe {
            LLVMSetValueName2(value, self.pool.c_str(name), name.len())
        }
    }

    pub fn add_function(&mut self, name: &str, fn_ty: LLVMTypeRef) -> Result<LLVMValueRef> {
        unsafe {
            Self::ptr_to_result(
                LLVMAddFunction(self.module, self.pool.c_str(name), fn_ty)
            )
        }
    }

    pub fn get_named_function(&mut self, name: &str) -> Result<LLVMValueRef> {
        unsafe {
            Self::ptr_to_result(
                LLVMGetNamedFunction(self.module, self.pool.c_str(name))
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

    pub fn count_params(fn_value: LLVMValueRef) -> usize {
        unsafe {
            LLVMCountParams(fn_value) as usize
        }
    }

    pub fn append_basic_block(&mut self, fn_value: LLVMValueRef, name: &str) -> Result<LLVMBasicBlockRef> {
        unsafe {
            Self::ptr_to_result(
                LLVMAppendBasicBlock(fn_value, self.pool.c_str(name))
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

    pub fn build_call(&mut self, fn_ty: LLVMTypeRef, fn_value: LLVMValueRef, args: Vec<LLVMValueRef>, name: &str) -> Result<LLVMValueRef> {
        unsafe {
            let arg_count = args.len() as u32;
            let arg_vec = self.pool.ptr_vec(args);
            Self::ptr_to_result(
                LLVMBuildCall2(self.builder, fn_ty, fn_value, arg_vec, arg_count, self.pool.c_str(name))
            )
        }
    }

    pub fn build_add(&mut self, lhs: LLVMValueRef, rhs: LLVMValueRef, name: &str) -> Result<LLVMValueRef> {
        unsafe {
            Self::ptr_to_result(
                LLVMBuildAdd(self.builder, lhs, rhs, self.pool.c_str(name))
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
