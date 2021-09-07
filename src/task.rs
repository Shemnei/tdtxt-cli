use std::fs::File;
use std::io::{self, BufRead, BufReader, Lines};
use std::iter::Enumerate;
use std::path::Path;
use std::str::FromStr;

use anyhow::Context;
use tdtxt::{ParseTaskError, Task};

pub struct TaskIter {
	lines: Enumerate<Lines<BufReader<File>>>,
}

impl TaskIter {
	pub fn new(path: &Path) -> io::Result<Self> {
		Ok(Self {
			lines: BufReader::new(File::open(path)?).lines().enumerate(),
		})
	}
}

impl Iterator for TaskIter {
	type Item = (usize, anyhow::Result<Task>);

	fn next(&mut self) -> Option<Self::Item> {
		let (l_nr, l_str) = self.lines.next()?;
		match l_str {
			Err(err) => Some((l_nr, Err(anyhow::anyhow!(err)))),
			Ok(l_str) => Some((
				l_nr,
				Task::from_str(&l_str)
					.with_context(|| "failed to parse line as task"),
			)),
		}
	}
}
