use std::{
    str::{
        CharIndices,
        Lines,
    },
    iter::Peekable
};

use crate::data::*;

#[derive(Clone, Debug)]
pub struct StrInfoIter<'a>
{
    s: &'a str,
    indices: CharIndices<'a>,
    next_indices: CharIndices<'a>,
    lines: Peekable<Lines<'a>>,
    line: usize,
    column: usize,
}

impl<'a> StrInfoIter<'a>
{
    pub fn new(s: &'a str) -> Self {
        let indices = s.char_indices();
        let mut next_indices = s.char_indices();
        next_indices.next();
        let lines = s.lines().peekable();
        Self {
            s,
            indices,
            next_indices,
            lines,
            line: 1,
            column: 0,
        }
    }

    fn next_str_info(&mut self, slice: &'a str, line_slice: &'a str, c: char) -> StrInfo<'a> {
        if c == '\n' {
            self.line += 1;
            self.column = 0;
        }
        else {
            self.column += 1;
        }
        StrInfo::new(self.line, self.column, slice, line_slice)
    }
}

impl<'a> Iterator for StrInfoIter<'a>
{
    type Item = (StrInfo<'a>, char);

    fn next(&mut self) -> Option<Self::Item> {
        let index = self.indices.next();
        let next_index = self.next_indices.next();
        let line_slice = self.lines.peek();
        let slice = match (index, next_index) {
            (Some((index, _)), Some((next_index, _))) =>
                Some(&self.s[index .. next_index]),
            (Some((index, _)), None) =>
                Some(&self.s[index ..]),
            _ =>
                None,
        };
        match slice {
            Some(slice) => {
                let line_slice = *line_slice.unwrap();
                let c = slice.chars().next().unwrap();
                if c == '\n' {
                    self.lines.next();
                }
                Some((self.next_str_info(slice, line_slice, c), c))
            },
            None => None,
        }
    }
}
