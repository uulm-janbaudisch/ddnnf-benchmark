use std::{
    io::{self, Write},
    path::{Path, PathBuf},
    process::Command,
    time::Instant,
};

use tempfile::NamedTempFile;

use crate::{Milliseconds, compiler::CompilerTool, tools::CliTool};

pub struct D4(PathBuf);

impl CompilerTool for D4 {
    fn new(path: Option<PathBuf>) -> io::Result<Self>
    where
        Self: Sized,
    {
        Ok(Self(path.unwrap_or(PathBuf::from("d4"))))
    }

    fn compile(&self, cnf: &Path) -> io::Result<(NamedTempFile, Milliseconds)> {
        let output_file = NamedTempFile::new().expect("Failed to create output file");

        let mut command = Command::new(&self.0);
        let command = command.args([
            "--method",
            "ddnnf-compiler",
            "--input",
            cnf.to_str().expect("Failed to serialize d-DNNF path"),
            "--dump-ddnnf",
            output_file
                .path()
                .to_str()
                .expect("Failed to serialize output file path"),
        ]);

        let start = Instant::now();
        let output = command.output()?;
        let time = start.elapsed().as_millis();

        if !output.status.success() {
            io::stdout().write_all(&output.stdout)?;
            io::stderr().write_all(&output.stderr)?;
            return Err(io::Error::other("Failed to compile d-DNNF"));
        }

        Ok((output_file, time))
    }
}

impl CliTool for D4 {
    fn is_working(&self) -> io::Result<bool> {
        Ok(Command::new(&self.0)
            .args(["--help"])
            .output()?
            .status
            .success())
    }
}
