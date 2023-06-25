mod iter;

pub use iter::*;

use std::{
    ptr,
    str,
};

#[derive(Clone, Debug)]
pub struct StrInfo<'a> {
    pub line: usize,
    pub column: usize,
    pub slice: &'a str,
    pub line_slice: &'a str,
}

#[cfg(not(test))]
impl<'a> PartialEq for StrInfo<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.line == other.line &&
        self.column == other.column &&
        self.slice == other.slice
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

#[cfg(test)]
impl<'a> PartialEq for StrInfo<'a> {
    fn eq(&self, _other: &Self) -> bool {
        true
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

impl<'a> StrInfo<'a> {
    pub fn new(line: usize, column: usize, slice: &'a str, line_slice: &'a str) -> Self {
        Self { line, column, slice, line_slice }
    }

    pub fn extend(&self, tail: &Self) -> Self {
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
        format!("{}\n{}", self.line_slice, self.underline())
    }

    fn underline(&self) -> String {
        let slice_head = self.slice.as_ptr() as usize;
        let line_head = self.line_slice.as_ptr() as usize;
        let spaces = (0 .. (slice_head - line_head)).map(|_| ' ').collect::<String>();
        let underline =
            if spaces.len() + self.slice.len() < self.line_slice.len() {
                (0 .. self.slice.len()).map(|_| '^').collect::<String>()
            }
            else if spaces.len() < self.line_slice.len() {
                (0 .. self.line_slice.len() - spaces.len()).map(|_| '^').collect::<String>()
            }
            else {
                String::new()
            };
        format!("{}{}", spaces, underline)
    }
}
