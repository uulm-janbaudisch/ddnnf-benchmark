use std::{fmt, path::Path};

use crate::{Count, Milliseconds};

static CSV_HEADER: &str = "model,compile_time,count,count_time,t_wise_time";

pub struct Runs(Vec<Run>);

#[derive(Debug, Clone)]
pub struct Run {
    pub model: String,
    pub compile_time: Milliseconds,
    pub count: Count,
    pub count_time: Milliseconds,
    pub t_wise_time: Milliseconds,
}

impl fmt::Display for Runs {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{CSV_HEADER}")?;
        self.0.iter().try_for_each(|run| {
            writeln!(f)?;
            write!(f, "{run}")
        })
    }
}

impl fmt::Display for Run {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{},{},{},{},{}",
            self.model, self.compile_time, self.count, self.count_time, self.t_wise_time
        )
    }
}

impl FromIterator<Run> for Runs {
    fn from_iter<T: IntoIterator<Item = Run>>(iter: T) -> Self {
        Self(iter.into_iter().collect())
    }
}

pub fn model_name(path: impl AsRef<Path>) -> String {
    path.as_ref()
        .file_stem()
        .expect("File name should not be empty")
        .to_str()
        .expect("Failed to convert file name")
        .to_string()
}
