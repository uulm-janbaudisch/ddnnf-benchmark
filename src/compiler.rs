pub mod d4;

use std::{io, path::Path};

use clap::ValueEnum;
use tempfile::NamedTempFile;

use crate::{Args, Milliseconds};

#[derive(Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum Compiler {
    D4,
}

pub trait CompilerTool {
    fn new(args: &Args) -> io::Result<Self>
    where
        Self: Sized;
    fn is_working(&self) -> io::Result<bool>;
    fn compile(&self, cnf: &Path) -> io::Result<(NamedTempFile, Milliseconds)>;
}
