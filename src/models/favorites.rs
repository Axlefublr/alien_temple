use crate::extra::DATE_FORMAT;
use crate::extra::FAVORITE_FILE;
use chrono::Local;
use std::fs;

pub struct FavoriteRepo {
	contents: String,
}

impl FavoriteRepo {
	pub fn new() -> Result<Self, &'static str> {
		let contents = parse()?;
		Ok(Self { contents })
	}

	pub fn has(&self, artist: &str) -> bool {
		self.contents
			.lines()
			.any(|line| line.split(" — ").nth(1).unwrap_or_default() == artist)
	}

	pub fn add(mut self, artist: &str, timestamp: &Option<String>) -> Result<(), &'static str> {
		let today = match timestamp {
			Some(timestamp) => timestamp.to_owned(),
			None => Local::now().format(DATE_FORMAT).to_string(),
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

	pub fn remove(mut self, artist: &str) -> Result<(), &'static str> {
		self.contents = self
			.contents
			.lines()
			.filter(|line| line.split(" — ").nth(1).unwrap_or_default() != artist)
			.map(|line| line.to_owned())
			.collect::<Vec<_>>()
			.join("\n");
		self.save()
	}

	fn save(self) -> Result<(), &'static str> {
		fs::write(FAVORITE_FILE, self.contents).map_err(|_| "couldn't write to favorites file")
	}
}

fn parse() -> Result<String, &'static str> {
	let contents = fs::read_to_string(FAVORITE_FILE).map_err(|_| "couldn't read favorites file")?;
	Ok(contents)
}
