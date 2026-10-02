pub mod d4;

use std::{
    io,
    path::{Path, PathBuf},
};

use clap::ValueEnum;
use tempfile::NamedTempFile;

use crate::{Milliseconds, tools::CliTool};

#[derive(Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum Compiler {
    D4,
}

pub trait CompilerTool
where
    Self: CliTool,
{
    fn new(path: Option<PathBuf>) -> io::Result<Self>
    where
        Self: Sized;
    fn compile(&self, cnf: &Path) -> io::Result<(NamedTempFile, Milliseconds)>;
}
