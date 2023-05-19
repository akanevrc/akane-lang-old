mod data;
mod lexer;
mod parser;
mod llvm;
mod codegen;

use std::fs;
use anyhow::Result;
use clap::Parser;
use self::llvm::LLVM;

#[derive(Parser, Debug)]
#[command(name = "akanec", author, version, about, long_about = None)]
struct Args {
    /// Input file path
    input: String,

    /// Output file path
    #[arg(short, long, default_value = "./a.ll")]
    output: String,
}

fn main() -> Result<()> {
    let args = Args::parse();
    compile(&args.input, &args.output)?;
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
