pub mod ddnnife;

use std::{
    io,
    path::{Path, PathBuf},
};

use clap::ValueEnum;

use crate::{
    Count, Milliseconds,
    compiler::CompilerTool,
    run::{Run, Runs, model_name},
};

static TWISE_T: usize = 2;

#[derive(Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum Tool {
    Ddnnife,
}

pub trait CliTool {
    fn is_working(&self) -> io::Result<bool>;
}

pub trait DdnnfTool
where
    Self: CliTool,
{
    fn new(path: Option<PathBuf>) -> io::Result<Self>
    where
        Self: Sized;
    fn count(&self, ddnnf: &Path) -> io::Result<(Count, Milliseconds)>;
    fn t_wise(&self, ddnnf: &Path, t: usize) -> io::Result<Milliseconds>;

    fn run_single(&self, cnf: &Path, compiler: &Box<dyn CompilerTool>) -> io::Result<Run> {
        let (ddnnf, d4_time) = compiler.compile(cnf)?;

        let (count, count_time) = self.count(ddnnf.path())?;
        let t_wise_time = self.t_wise(ddnnf.path(), TWISE_T)?;

        Ok(Run {
            model: model_name(cnf),
            compile_time: d4_time,
            count,
            count_time,
            t_wise_time,
        })
    }

    fn run(&self, input: &[PathBuf], compiler: &Box<dyn CompilerTool>) -> io::Result<Runs> {
        input
            .iter()
            .map(|path| self.run_single(path, compiler))
            .collect()
    }
}
