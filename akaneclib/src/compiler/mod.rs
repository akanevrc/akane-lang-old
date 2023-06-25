use std::fs;
use anyhow::Error;
use crate::{
    data::*,
    lexer,
    parser,
    semantizer,
    codegen,
};

pub fn compile(in_path: &str, out_path: &str) -> Result<(), Vec<Error>> {
    let code = fs::read_to_string(in_path).map_err(|e| vec![Error::from(e)])?;
    let tokens = lexer::lex(&code)?;
    let mut asts = parser::parse(tokens)?;
    let mut ctx = SemContext::new();
    semantizer::semantize(&mut ctx, &mut asts)?;
    let mut llvm = LLVM::new(in_path);
    codegen::compile(&mut llvm, &ctx, &asts).map_err(|e| vec![e])?;
    llvm.print_module_to_file(out_path).map_err(|e| vec![e])?;
    Ok(())
}
