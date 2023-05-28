use std::fs;
use anyhow::Result;
use crate::{
    data::{
        context::SemContext,
        llvm::LLVM,
    },
    lexer,
    parser,
    semantizer,
    codegen,
};

pub fn compile(in_path: &str, out_path: &str) -> Result<()> {
    let code = fs::read_to_string(in_path)?;
    let tokens = lexer::lex(code)?;
    let mut asts = parser::parse(tokens)?;
    let mut ctx = SemContext::new();
    semantizer::semantize(&mut ctx, &mut asts)?;
    let mut llvm = LLVM::new(in_path);
    codegen::compile(&mut llvm, &asts)?;
    llvm.print_module_to_file(out_path)?;
    Ok(())
}
