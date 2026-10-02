use std::{
    fs,
    io::{self, Write},
    path::{Path, PathBuf},
    process::Command,
    str::FromStr,
    time::Instant,
};

use num::BigUint;
use tempfile::NamedTempFile;

use crate::{
    Count, Milliseconds,
    tools::{CliTool, DdnnfTool},
};

pub struct Ddnnife(PathBuf);

impl Ddnnife {
    fn run(&self, ddnnf: &Path, args: &[&str]) -> io::Result<(NamedTempFile, Milliseconds)> {
        let output_file = NamedTempFile::new().expect("Failed to create output file");

        let mut command = Command::new(&self.0);
        let command = command.args(
            [
                &[
                    "--input",
                    ddnnf.to_str().expect("Failed to serialize d-DNNF path"),
                    "--output",
                    output_file
                        .path()
                        .to_str()
                        .expect("Failed to serialize output file path"),
                ],
                args,
            ]
            .concat(),
        );

        let start = Instant::now();
        let output = command.output()?;
        let time = start.elapsed().as_millis();

        if !output.status.success() {
            io::stdout().write_all(&output.stdout)?;
            io::stderr().write_all(&output.stderr)?;
            return Err(io::Error::other("Failed to run ddnnife"));
        }

        Ok((output_file, time))
    }
}

impl CliTool for Ddnnife {
    fn is_working(&self) -> io::Result<bool> {
        Ok(Command::new(&self.0)
            .args(["--help"])
            .output()?
            .status
            .success())
    }
}

impl DdnnfTool for Ddnnife {
    fn new(path: Option<PathBuf>) -> io::Result<Self> {
        Ok(Self(path.unwrap_or(PathBuf::from("ddnnife"))))
    }

    fn count(&self, ddnnf: &Path) -> io::Result<(Count, Milliseconds)> {
        let (output, time) = self.run(ddnnf, &["count"])?;
        let count = BigUint::from_str(&fs::read_to_string(output)?).expect("Failed to parse count");
        Ok((count, time))
    }

    fn t_wise(&self, ddnnf: &Path, t: usize) -> io::Result<Milliseconds> {
        let (_output, time) = self.run(ddnnf, &["t-wise", "-t", &t.to_string()])?;
        Ok(time)
    }
}
