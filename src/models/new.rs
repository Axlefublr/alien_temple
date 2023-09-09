use crate::extra::DATE_TIME_FORMAT;
use crate::extra::NEW_FILE;
use chrono::Local;
use std::fs;

pub struct NewRepo {
	contents: String,
}

impl NewRepo {
	pub fn new() -> Result<Self, &'static str> {
		let contents = parse()?;
		Ok(Self { contents })
	}

	pub fn peek(&self) -> Result<String, &'static str> {
		let artist = self
			.contents
			.lines()
			.next()
			.ok_or("new file doesn't contain a single line")?
			.split(" — ")
			.nth(1)
			.ok_or("new file has a line that isn't separated by —")?
			.to_owned();
		Ok(artist)
	}

	pub fn remove_first(mut self) -> Result<String, &'static str> {
		let artist = self.peek()?;
		self.contents = self
			.contents
			.lines()
			.skip(1)
			.map(|line| line.to_owned())
			.collect::<Vec<_>>()
			.join("\n");
		self.save()?;
		Ok(artist)
	}

	pub fn has(&self, artist: &str) -> bool {
		self.contents
			.lines()
			.any(|line| line.split(" — ").nth(1).unwrap_or_default() == artist)
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
		fs::write(NEW_FILE, self.contents).map_err(|_| "couldn't write to new file")
	}
}

fn parse() -> Result<String, &'static str> {
	let contents = fs::read_to_string(NEW_FILE).map_err(|_| "couldn't read new file")?;
	Ok(contents)
}
