use std::process::ExitCode;

use args::Args;
use args::UserCommands;
use clap::Parser;

mod actions;
mod args;
mod sh;
mod models;
mod extra;

fn main() -> ExitCode {
	let args = Args::parse();
	let should_commit = args.git;
	match args.action {
		UserCommands::Consent => actions::consent(),
		UserCommands::Touch { track, timestamp } => actions::touch(track, timestamp, should_commit),
		UserCommands::Interest { name } => unimplemented!(),
		UserCommands::Shark => unimplemented!(),
		UserCommands::Rotate => unimplemented!(),
		// UserCommands::Finish { name } => unimplemented!(),
		// UserCommands::Discover { name } => unimplemented!(),
		// UserCommands::Favorite { name } => unimplemented!(),
		UserCommands::Whomst => unimplemented!(),
		// UserCommands::Playlist { name, track } => unimplemented!(),
		_ => panic!(),
	}
}
