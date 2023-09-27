use std::fs;

use chrono::Local;

use crate::extra::DATE_TIME_FORMAT;
use crate::extra::DISCOVERY_FILE;

pub struct DiscoveryRepo {
	contents: String,
}

impl DiscoveryRepo {
	pub fn new() -> Result<Self, &'static str> {
		let contents = parse()?;
		Ok(Self { contents })
	}

	pub fn add(mut self, artist: &str, timestamp: &Option<String>) -> Result<(), &'static str> {
		let today = match timestamp {
			Some(timestamp) => timestamp.to_owned(),
			None => Local::now().format(DATE_TIME_FORMAT).to_string(),
		};
		let mut lines = self
			.contents
			.lines()
			.map(|line| line.to_owned())
			.collect::<Vec<_>>();
		lines.push(format!("{} — {}", today, artist));
		self.contents = lines.join("\n");
		self.save()
	}

	fn save(self) -> Result<(), &'static str> {
		fs::write(DISCOVERY_FILE, self.contents).map_err(|_| "couldn't write to discovery file")
	}
}

fn parse() -> Result<String, &'static str> {
	let contents = fs::read_to_string(DISCOVERY_FILE).map_err(|_| "couldn't read discovery file")?;
	Ok(contents)
}
