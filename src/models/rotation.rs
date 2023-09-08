use std::fs;

use chrono::Utc;

use crate::extra::DATE_TIME_FORMAT;
use crate::extra::ROTATE_FILE;

pub struct ArtistTrack {
	pub timestamp: String,
	pub artist: String,
	pub track: String,
}

pub struct RotateRepo {
	contents: String,
}

impl RotateRepo {
	pub fn new() -> Result<Self, &'static str> {
		let contents = parse()?;
		Ok(Self { contents })
	}

	pub fn add(mut self, artist: &str, track: &str) -> Result<(), &'static str> {
		let today = Utc::now().format(DATE_TIME_FORMAT);
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
		self.contents
			.lines()
			.any(|line| line.split(" — ").nth(1).unwrap_or_default() == artist)
	}

	pub fn peek(&self) -> Result<ArtistTrack, &'static str> {
		let mut artist_track = self
			.contents
			.lines()
			.next()
			.ok_or("rotate file has no lines")?
			.split(" — ");
		let error_msg = "rotate file doesn't use —";
		let timestamp = artist_track.next().ok_or(error_msg)?.to_owned();
		let artist = artist_track.next().ok_or(error_msg)?.to_owned();
		let track = artist_track.next().ok_or(error_msg)?.to_owned();
		Ok(ArtistTrack {
			timestamp,
			artist,
			track,
		})
	}

	pub fn rotate(mut self, track: &str) -> Result<String, &'static str> {
		let artist_track = self.peek()?;
		let mut other_lines = self
			.contents
			.lines()
			.skip(1)
			.map(|line| line.to_owned())
			.collect::<Vec<_>>();
		other_lines.push(format!(
			"{} — {} — {}",
			artist_track.timestamp, artist_track.artist, track
		));
		self.contents = other_lines.join("\n");
		self.save()?;
		Ok(artist_track.artist)
	}

	pub fn remove_first(mut self) -> Result<String, &'static str> {
		let artist_track = self.peek()?;
		let other_lines = self
			.contents
			.lines()
			.skip(1)
			.map(|line| line.to_owned())
			.collect::<Vec<_>>();
		self.contents = other_lines.join("\n");
		self.save()?;
		Ok(artist_track.artist)
	}

	fn save(self) -> Result<(), &'static str> {
		fs::write(ROTATE_FILE, self.contents).map_err(|_| "couldn't write to rotate file")
	}
}

fn parse() -> Result<String, &'static str> {
	let contents = fs::read_to_string(ROTATE_FILE).map_err(|_| "couldn't read rotate file")?;
	Ok(contents)
}
