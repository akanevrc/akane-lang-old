use std::{
    collections::HashMap,
    marker::PhantomData,
    pin::Pin,
};
use crate::data::*;

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
