use std::process::ExitCode;

use crate::models::new::NewRepo;
use crate::models::rotation::RotateRepo;

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

pub fn touch(track: String, timestamp: Option<String>, should_commit: bool) -> ExitCode {
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
	if let Err(message) = new_repo.kill_first() {
		eprintln!("{}", message);
		return ExitCode::FAILURE;
	}
	if let Err(message) = rotate_repo.add(&artist, &track, &timestamp) {
		eprintln!("{}", message);
		return ExitCode::FAILURE;
	};
	if should_commit {
		unimplemented!(); // todo: git commit
	}
	ExitCode::SUCCESS
}

pub fn interest(name: String, timestamp: Option<String>, should_commit: bool) -> ExitCode {
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
	if should_commit {
		unimplemented!(); // todo: git commit
	}
	ExitCode::SUCCESS
}