use std::{
    any::Any,
    collections::HashMap,
    ffi::{
        CStr,
        CString,
    },
};

pub struct Pool {
    c_strs: HashMap<*mut dyn Any, Box<CStr>>,
    ptr_vecs: HashMap<*mut dyn Any, Vec<*mut dyn Any>>,
}

impl Pool {
    pub fn new() -> Self {
        Self {
            c_strs: HashMap::new(),
            ptr_vecs: HashMap::new(),
        }
    }

    pub fn c_str(&mut self, s: &str) -> *const i8 {
        let mut s = CString::new(s).unwrap().into_boxed_c_str();
        let p: *mut dyn Any = &mut s;
        self.c_strs.insert(p, s);
        self.c_strs.get(&p).unwrap().as_ptr()
    }

    pub fn ptr_vec<T: 'static>(&mut self, ptrs: Vec<*mut T>) -> *mut *mut T {
        let mut v = Vec::<*mut dyn Any>::new();
        let p: *mut dyn Any = &mut v;
        for ptr in ptrs {
            v.push(ptr as *mut dyn Any);
        }
        self.ptr_vecs.insert(p, v);
        self.ptr_vecs.get_mut(&p).unwrap().as_mut_ptr() as *mut *mut T
    }
}
