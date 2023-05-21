use std::fs;
use anyhow::Result;
use crate::{
    lexer,
    parser,
    llvm::LLVM,
    codegen,
};

pub fn compile(in_path: &str, out_path: &str) -> Result<()> {
    let code = fs::read_to_string(in_path)?;
    let tokens = lexer::lex(code)?;
    let asts = parser::parse(tokens)?;
    let mut llvm = LLVM::new(in_path);
    codegen::compile(&mut llvm, &asts)?;
    llvm.print_module_to_file(out_path)?;
    Ok(())
}
