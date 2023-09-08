use std::process::ExitCode;

use crate::models::discovery::DiscoveryRepo;
use crate::models::new::NewRepo;
use crate::models::rotation::RotateRepo;
use crate::sh::git_add_commit;

pub fn consent() -> ExitCode {
	let new_repo = match NewRepo::new() {
		Ok(repo) => repo,
		Err(message) => {
			eprintln!("{}", message);
			return ExitCode::FAILURE;
		}
	};
	match new_repo.peek() {
		Ok(artist) => {
			println!("{}", artist);
			ExitCode::SUCCESS
		},
		Err(message) => {
			eprintln!("{}", message);
			ExitCode::FAILURE
		}
	}
}

pub fn touch(track: String) -> ExitCode {
	let new_repo = match NewRepo::new() {
		Ok(repo) => repo,
		Err(message) => {
			eprintln!("{}", message);
			return ExitCode::FAILURE;
		}
	};
	let rotate_repo = match RotateRepo::new() {
		Ok(repo) => repo,
		Err(message) => {
			eprintln!("{}", message);
			return ExitCode::FAILURE;
		}
	};
	let artist = match new_repo.peek() {
		Ok(artist) => artist,
		Err(message) => {
			eprintln!("{}", message);
			return ExitCode::FAILURE;
		}
	};
	if let Err(message) = new_repo.remove_first() {
		eprintln!("{}", message);
		return ExitCode::FAILURE;
	}
	if let Err(message) = rotate_repo.add(&artist, &track) {
		eprintln!("{}", message);
		return ExitCode::FAILURE;
	};
	if let Err(message) = git_add_commit(&format!("touch {}", &artist)) {
		eprintln!("{}", message);
		return ExitCode::FAILURE;
	}
	ExitCode::SUCCESS
}

pub fn tinish() -> ExitCode {
	let new_repo = match NewRepo::new() {
		Ok(repo) => repo,
		Err(message) => {
			eprintln!("{}", message);
			return ExitCode::FAILURE;
		}
	};
	let artist = match new_repo.remove_first() {
		Ok(artist) => artist,
		Err(message) => {
			eprintln!("{}", message);
			return ExitCode::FAILURE;
		}
	};
	if let Err(message) = git_add_commit(&format!("touch & finish {}", &artist)) {
		eprintln!("{}", message);
		return ExitCode::FAILURE;
	}
	ExitCode::SUCCESS
}

pub fn interest(name: String, timestamp: Option<String>) -> ExitCode {
	let new_repo = match NewRepo::new() {
		Ok(repo) => repo,
		Err(message) => {
			eprintln!("{}", message);
			return ExitCode::FAILURE;
		}
	};
	if new_repo.has(&name) {
		eprintln!("{} is already in your new list!", name);
		return ExitCode::FAILURE;
	}
	let rotation_repo = match RotateRepo::new() {
		Ok(repo) => repo,
		Err(message) => {
			eprintln!("{}", message);
			return ExitCode::FAILURE;
		}
	};
	if rotation_repo.has(&name) {
		eprintln!("{} is already in your rotation list!", name);
		return ExitCode::FAILURE;
	}
	if let Err(message) = new_repo.add(&name, &timestamp) {
		eprintln!("{}", message);
		return ExitCode::FAILURE;
	}
	if let Err(message) = git_add_commit(&format!("interest {}", &name)) {
		eprintln!("{}", message);
		return ExitCode::FAILURE;
	}
	ExitCode::SUCCESS
}

pub fn discover(name: String, timestamp: Option<String>) -> ExitCode {
	let discovery_repo = match DiscoveryRepo::new() {
		Ok(repo) => repo,
		Err(message) => {
			eprintln!("{}", message);
			return ExitCode::FAILURE;
		}
	};
	if let Err(message) = discovery_repo.add(&name, &timestamp) {
		eprintln!("{}", message);
		return ExitCode::FAILURE;
	}
	if let Err(message) = git_add_commit(&format!("discover {}", &name)) {
		eprintln!("{}", message);
		return ExitCode::FAILURE;
	}
	ExitCode::SUCCESS
}

pub fn shark() -> ExitCode {
	let rotate_repo = match RotateRepo::new() {
		Ok(repo) => repo,
		Err(message) => {
			eprintln!("{}", message);
			return ExitCode::FAILURE;
		}
	};
	match rotate_repo.peek() {
		Ok(artist_track) => {
			println!("{}", artist_track.artist);
			println!("{}", artist_track.track);
			ExitCode::SUCCESS
		},
		Err(message) => {
			eprintln!("{}", message);
			ExitCode::FAILURE
		}
	}
}

pub fn rotate(track: String) -> ExitCode {
	let rotate_repo = match RotateRepo::new() {
		Ok(repo) => repo,
		Err(message) => {
			eprintln!("{}", message);
			return ExitCode::FAILURE;
		}
	};
	let artist = match rotate_repo.rotate(&track) {
		Ok(artist) => artist,
		Err(message) => {
			eprintln!("{}", message);
			return ExitCode::FAILURE;
		}
	};
	if let Err(message) = git_add_commit(&format!("rotate {}", &artist)) {
		eprintln!("{}", message);
		return ExitCode::FAILURE;
	}
	ExitCode::SUCCESS
}

pub fn finish() -> ExitCode {
	let rotate_repo = match RotateRepo::new() {
		Ok(repo) => repo,
		Err(message) => {
			eprintln!("{}", message);
			return ExitCode::FAILURE;
		}
	};
	let artist = match rotate_repo.remove_first() {
		Ok(artist) => artist,
		Err(message) => {
			eprintln!("{}", message);
			return ExitCode::FAILURE;
		}
	};
	if let Err(message) = git_add_commit(&format!("rotate & finish {}", &artist)) {
		eprintln!("{}", message);
		return ExitCode::FAILURE;
	}
	ExitCode::SUCCESS
}

pub fn uninterest() -> ExitCode {
	let rotate_repo = match RotateRepo::new() {
		Ok(repo) => repo,
		Err(message) => {
			eprintln!("{}", message);
			return ExitCode::FAILURE;
		}
	};
	let artist = match rotate_repo.remove_first() {
		Ok(artist) => artist,
		Err(message) => {
			eprintln!("{}", message);
			return ExitCode::FAILURE;
		}
	};
	if let Err(message) = git_add_commit(&format!("uninterest {}", &artist)) {
		eprintln!("{}", message);
		return ExitCode::FAILURE;
	}
	ExitCode::SUCCESS
}