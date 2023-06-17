
#[derive(Clone, Debug, PartialEq)]
pub struct StrInfo<'a> {
    pub line: usize,
    pub column: usize,
    pub slice: &'a str,
}

impl<'a> StrInfo<'a> {
    pub fn new(line: usize, column: usize, slice: &'a str) -> Self {
        Self { line, column, slice }
    }
}
