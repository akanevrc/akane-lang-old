use std::str::CharIndices;
use crate::data::*;

#[derive(Clone, Debug)]
pub struct StrInfoIter<'a>
{
    s: &'a str,
    indices: CharIndices<'a>,
    next_indices: CharIndices<'a>,
    line: usize,
    column: usize,
}

impl<'a> StrInfoIter<'a>
{
    pub fn new(s: &'a str) -> Self {
        let indices = s.char_indices();
        let mut next_indices = s.char_indices();
        next_indices.next();
        Self {
            s,
            indices,
            next_indices,
            line: 1,
            column: 0,
        }
    }

    fn next_str_info(&mut self, s: &'a str, c: char) -> StrInfo<'a> {
        if c == '\n' {
            self.line += 1;
            self.column = 0;
        }
        else {
            self.column += 1;
        }
        StrInfo::new(self.line, self.column, s)
    }
}

impl<'a> Iterator for StrInfoIter<'a>
{
    type Item = (StrInfo<'a>, char);

    fn next(&mut self) -> Option<Self::Item> {
        let index = self.indices.next();
        let next_index = self.next_indices.next();
        match (index, next_index) {
            (Some((index, _)), Some((next_index, _))) => {
                let s = &self.s[index .. next_index];
                let c = s.chars().next().unwrap();
                Some((self.next_str_info(s, c), c))
            },
            (Some((index, _)), None) => {
                let s = &self.s[index ..];
                let c = s.chars().next().unwrap();
                Some((self.next_str_info(s, c), c))
            },
            _ =>
                None,
        }
    }
}
