mod iter;

pub use iter::*;

use std::{
    ptr,
    str,
};

#[derive(Clone, Debug, PartialEq)]
pub struct StrInfo<'a> {
    pub line: usize,
    pub column: usize,
    pub slice: &'a str,
    pub line_slice: &'a str,
}

impl<'a> StrInfo<'a> {
    pub fn new(line: usize, column: usize, slice: &'a str, line_slice: &'a str) -> Self {
        Self { line, column, slice, line_slice }
    }

    pub fn extend(self, tail: Self) -> Self {
        let head = self.slice.as_ptr() as usize;
        let tail = tail.slice.as_ptr() as usize;
        let len = tail - head + 1;
        let slice = unsafe { ptr::slice_from_raw_parts(self.slice.as_ptr(), len).as_ref().unwrap() };
        let s = unsafe { str::from_utf8_unchecked(slice) };
        Self {
            line: self.line,
            column: self.column,
            slice: s,
            line_slice: self.line_slice,
        }
    }

    pub fn target_part_of_line(&self) -> String {
        format!("\t{}\n\t{}", self.line_slice, self.underline())
    }

    fn underline(&self) -> String {
        let slice_head = self.slice.as_ptr() as usize;
        let line_head = self.line_slice.as_ptr() as usize;
        let spaces = (0 .. (slice_head - line_head)).map(|_| ' ').collect::<String>();
        let underline =
            if self.slice.len() == 0 {
                String::new()
            }
            else {
                (0 .. self.slice.len()).map(|_| '^').collect::<String>()
            };
        format!("{}{}", spaces, underline)
    }
}
