use crate::extra::PLAYLIST_FILE;
use std::fmt::Display;
use std::fs;

const MAX_PLAYLIST: u32 = 50;
const STEP: u32 = 9;

pub struct PlaylistRepo {
	pub current: u32,
	pub artist: String,
	pub track: String,
}

impl PlaylistRepo {
	pub fn new() -> Result<Self, &'static str> {
		let contents = fs::read_to_string(PLAYLIST_FILE).map_err(|_| "couldn't read playlist file")?;
		let mut sections = contents.split(" — ");
		let err_msg = "playlist file doesn't use —";
		let current: u32 = sections
			.next()
			.ok_or(err_msg)?
			.parse()
			.map_err(|_| "couldn't parse first section of playlist file to a u32")?;
		let artist = sections
			.next()
			.ok_or(err_msg)?
			.to_owned();
		let track = sections
			.next()
			.ok_or(err_msg)?
			.to_owned();
		Ok(Self {
			current,
			artist,
			track,
		})
	}

	fn next_playlist(&self) -> u32 {
		let next = self.current + STEP;
		match next > MAX_PLAYLIST {
			true => next - MAX_PLAYLIST,
			false => next,
		}
	}

	pub fn update(mut self, artist: &str, track: &str) -> Result<(), &'static str> {
		self.artist = artist.to_owned();
		self.track = track.to_owned();
		self.save()
	}

	pub fn next(mut self) -> Result<(), &'static str> {
		let next = self.next_playlist();
		self.current = next;
		println!("{}", next);
		self.save()
	}

	fn save(self) -> Result<(), &'static str> {
		fs::write(PLAYLIST_FILE, self.to_string()).map_err(|_| "couldn't write to playlist file")
	}
}

impl Display for PlaylistRepo {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{} — {} — {}", self.current, self.artist, self.track)
	}
}
