use std::{io, path::Path, time::Instant};

use ddnnife::Ddnnf;

use crate::{Args, Count, Milliseconds, tools::DdnnfTool};

pub struct Ddnnife(Option<Ddnnf>);

impl DdnnfTool for Ddnnife {
    fn new(_: &Args) -> io::Result<Self> {
        Ok(Self(None))
    }

    fn is_working(&self) -> io::Result<bool> {
        Ok(true)
    }

    fn load(&mut self, ddnnf: &Path) -> io::Result<Milliseconds> {
        let start = Instant::now();
        self.0 = Some(Ddnnf::from_file(ddnnf, None));
        let time = start.elapsed().as_millis();
        Ok(time)
    }

    fn count(&self) -> io::Result<(Count, Milliseconds)> {
        let start = Instant::now();
        let count = self.0.as_ref().unwrap().rc();
        let time = start.elapsed().as_millis();
        Ok((count, time))
    }

    fn t_wise(&self, t: usize) -> io::Result<Milliseconds> {
        let start = Instant::now();
        let _ = self.0.as_ref().unwrap().sample_t_wise(t, None);
        let time = start.elapsed().as_millis();
        Ok(time)
    }
}
