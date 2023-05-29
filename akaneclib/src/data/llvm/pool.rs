use std::{
    collections::HashMap,
    ffi::{
        CStr,
        CString,
    },
    marker::PhantomData,
    pin::Pin,
};
use super::Ptr;

pub struct CStrPool {
    c_strs: HashMap<*const i8, Pin<Box<CStr>>>,
}

impl CStrPool {
    pub fn new() -> Self {
        Self { c_strs: HashMap::new() }
    }

    pub fn c_str(&mut self, s: &str) -> *const i8 {
        let pinned = Pin::new(CString::new(s).unwrap().into_boxed_c_str());
        let p = pinned.as_ptr();
        self.c_strs.insert(p, pinned);
        p
    }
}

pub struct SlicePool<T, LLVMRef: Ptr<T> + Unpin> {
    slices: HashMap<*mut LLVMRef, Pin<Box<[LLVMRef]>>>,
    phantom: PhantomData<T>,
}

impl<T, LLVMRef: Ptr<T> + Unpin> SlicePool<T, LLVMRef> {
    pub fn new() -> Self {
        Self { slices: HashMap::new(), phantom: PhantomData }
    }

    pub fn slice(&mut self, ptrs: &[LLVMRef]) -> *mut LLVMRef {
        let mut pinned =
            Pin::new(
                ptrs.iter()
                .map(|ptr| *ptr)
                .collect::<Vec<_>>()
                .into_boxed_slice()
            );
        let p = pinned.as_mut_ptr();
        self.slices.insert(p, pinned);
        p
    }
}
