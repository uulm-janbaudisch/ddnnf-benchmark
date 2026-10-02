mod compiler;
mod run;
mod tools;

use std::{fs, io, path::PathBuf};

use clap::Parser;
use num::BigUint;
use tools::{DdnnfTool, Tool, ddnnife::Ddnnife};

use crate::compiler::{Compiler, CompilerTool, d4::D4};

type Count = BigUint;
type Milliseconds = u128;

/// Benchmarking tool for d-DNNF computations.
///
/// Supports different d-DNNF compilers and tools.
/// A run is defined by one combination of compiler and tool.
#[derive(Parser)]
struct Args {
    /// Path to d4.
    #[arg(long, env)]
    d4_path: Option<PathBuf>,

    /// Path to ddnnife.
    #[arg(long, env)]
    ddnnife_path: Option<PathBuf>,

    /// Which compiler to use.
    #[arg(short, long, env)]
    compiler: Compiler,

    /// Which tool to run.
    #[arg(short, long, env)]
    tool: Tool,

    /// Where to write the result CSV to.
    /// Will be overridden if it exists.
    #[arg(short, long, env)]
    output: PathBuf,

    /// Path to one or multiple input files.
    input: Vec<PathBuf>,
}

fn main() -> io::Result<()> {
    let args = Args::parse();

    let compiler: Box<dyn CompilerTool> = match args.compiler {
        Compiler::D4 => Box::new(D4::new(args.d4_path)?),
    };

    let tool: Box<dyn DdnnfTool> = match args.tool {
        Tool::Ddnnife => Box::new(Ddnnife::new(args.ddnnife_path)?),
    };

    assert!(compiler.is_working()?, "Compiler is not working");
    assert!(tool.is_working()?, "The chosen d-DNNF tool is not working");

    fs::write(args.output, tool.run(&args.input, &compiler)?.to_string())
}
