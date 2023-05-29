use anyhow::Result;
use clap::Parser;
use akaneclib::compiler;

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
    compiler::compile(&args.input, &args.output)?;
    Ok(())
}
