mod lexer;
mod parser;
mod llvm;
mod codegen;

use std::{
    env,
    fs
};
use anyhow::Result;
use llvm::LLVM;

fn main() -> Result<()> {
    let mut args = env::args().skip(1);
    let in_path = args.next().unwrap();
    let out_path = args.next().unwrap();
    compile(&in_path, &out_path)?;
    Ok(())
}

fn compile(in_path: &str, out_path: &str) -> Result<()> {
    let code = fs::read_to_string(in_path)?;
    let tokens = lexer::lex(code)?;
    let asts = parser::parse(tokens)?;
    let mut llvm = LLVM::new(in_path);
    codegen::compile(&mut llvm, &asts)?;
    llvm.print_module_to_file(out_path)?;
    Ok(())
}
