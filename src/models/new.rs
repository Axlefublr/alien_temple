use crate::extra::NEW_FILE;
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
			.ok_or("since file isn't empty, it should contain at least one line")?
			.split(" — ")
			.nth(1)
			.ok_or("new file has a line that isn't separated by —")?
			.to_owned();
		Ok(artist)
	}

	pub fn kill_first(mut self) -> Result<(), &'static str> {
		self.contents = self
			.contents
			.lines()
			.skip(1)
			.map(|line| line.to_owned())
			.collect::<Vec<_>>()
			.join("\n");
		self.save()
	}

	fn save(self) -> Result<(), &'static str> {
		fs::write(NEW_FILE, self.contents).map_err(|_| "couldn't write to new file")
	}
}

fn parse() -> Result<String, &'static str> {
	let contents = fs::read_to_string(NEW_FILE).map_err(|_| "couldn't read new file")?;
	if contents.is_empty() {
		return Err("no new rappers!");
	}
	Ok(contents)
}
