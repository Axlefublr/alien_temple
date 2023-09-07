use std::fs;

use chrono::Utc;

use crate::extra::DATE_TIME_FORMAT;
use crate::extra::ROTATE_FILE;

pub struct RotateRepo {
	contents: String,
}

impl RotateRepo {
	pub fn new() -> Result<Self, &'static str> {
		let contents = parse()?;
		Ok(Self { contents })
	}

	pub fn add(mut self, artist: &str, track: &str, timestamp: &Option<String>) -> Result<(), &'static str> {
		let today = match timestamp {
			Some(timestamp) => timestamp.to_owned(),
			None => Utc::now().format(DATE_TIME_FORMAT).to_string()
		};
		let mut lines = self
			.contents
			.lines()
			.map(|line| line.to_owned())
			.collect::<Vec<_>>();
		lines.push(format!("{} — {} — {}", today, artist, track));
		self.contents = lines.join("\n");
		self.save()
	}

	fn save(self) -> Result<(), &'static str> {
		fs::write(ROTATE_FILE, self.contents).map_err(|_| "couldn't write to rotate file")
	}
}

fn parse() -> Result<String, &'static str> {
	let contents = fs::read_to_string(ROTATE_FILE).map_err(|_| "couldn't read rotate file")?;
	Ok(contents)
}
