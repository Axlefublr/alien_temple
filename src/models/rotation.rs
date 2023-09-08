use std::fs;

use chrono::Utc;

use crate::extra::DATE_TIME_FORMAT;
use crate::extra::ROTATE_FILE;

pub struct ArtistTrack {
	pub artist: String,
	pub track: String
}

pub struct RotateRepo {
	contents: String,
}

impl RotateRepo {
	pub fn new() -> Result<Self, &'static str> {
		let contents = parse()?;
		Ok(Self { contents })
	}

	pub fn add(
		mut self,
		artist: &str,
		track: &str,
		timestamp: &Option<String>,
	) -> Result<(), &'static str> {
		let today = match timestamp {
			Some(timestamp) => timestamp.to_owned(),
			None => Utc::now().format(DATE_TIME_FORMAT).to_string(),
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

	pub fn has(&self, artist: &str) -> bool {
		self.contents.lines().any(|line| {
			line.split(" — ")
				.nth(1)
				.expect("rotate file should be using the — separator")
				== artist
		})
	}

	pub fn peek(&self) -> Result<ArtistTrack, &'static str> {
		let mut artist_pair = self.contents
			.lines()
			.next()
			.ok_or("rotate file has no lines")?
			.split(" — ")
			.skip(1);
		let artist = artist_pair.next().ok_or("rotate file doesn't use —")?.to_owned();
		let track = artist_pair.next().ok_or("rotate file doesn't use —")?.to_owned();
		Ok(ArtistTrack {
			artist,
			track
		})
	}

	fn save(self) -> Result<(), &'static str> {
		fs::write(ROTATE_FILE, self.contents).map_err(|_| "couldn't write to rotate file")
	}
}

fn parse() -> Result<String, &'static str> {
	let contents = fs::read_to_string(ROTATE_FILE).map_err(|_| "couldn't read rotate file")?;
	Ok(contents)
}
