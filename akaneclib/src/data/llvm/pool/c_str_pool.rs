use std::{
    collections::HashMap,
    ffi::{
        CStr,
        CString,
    },
    pin::Pin,
};

pub struct CStrPool {
    c_strs: HashMap<CString, Pin<Box<CStr>>>,
}

impl CStrPool {
    pub fn new() -> Self {
        Self { c_strs: HashMap::new() }
    }

    pub fn c_str(&mut self, s: &str) -> *const i8 {
        let c_string = CString::new(s).unwrap();
        let pinned = Pin::new(c_string.clone().into_boxed_c_str());
        let p = pinned.as_ptr();
        self.c_strs.insert(c_string, pinned);
        p
    }
}
