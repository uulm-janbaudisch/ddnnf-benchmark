pub mod ddnnife;

use std::{
    io,
    path::{Path, PathBuf},
};

use clap::ValueEnum;

use crate::{
    Args, Count, Milliseconds,
    compiler::CompilerTool,
    run::{Run, Runs, model_name},
};

static TWISE_T: usize = 2;

#[derive(Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum Tool {
    Ddnnife,
}

pub trait DdnnfTool {
    fn new(args: &Args) -> io::Result<Self>
    where
        Self: Sized;

    fn is_working(&self) -> io::Result<bool>;
    fn load(&mut self, ddnnf: &Path) -> io::Result<Milliseconds>;
    fn count(&self) -> io::Result<(Count, Milliseconds)>;
    fn t_wise(&self, t: usize) -> io::Result<Milliseconds>;

    fn run(&mut self, cnf: &Path, compiler: &dyn CompilerTool) -> io::Result<Run> {
        let (ddnnf, compile_time) = compiler.compile(cnf)?;
        let load_time = self.load(ddnnf.path())?;

        let (count, count_time) = self.count()?;
        let t_wise_time = self.t_wise(TWISE_T)?;

        Ok(Run {
            model: model_name(cnf),
            compile_time,
            load_time,
            count,
            count_time,
            t_wise_time,
        })
    }

    fn run_all(&mut self, input: &[PathBuf], compiler: &dyn CompilerTool) -> io::Result<Runs> {
        input.iter().map(|path| self.run(path, compiler)).collect()
    }
}
