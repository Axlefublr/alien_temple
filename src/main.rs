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
	match args.action {
		UserCommands::Consent => unimplemented!(),
		UserCommands::Rotate => unimplemented!(),
		UserCommands::Shark => unimplemented!(),
		UserCommands::Touch => unimplemented!(),
		UserCommands::Whomst => unimplemented!(),
		// UserCommands::Discover { name } => unimplemented!(),
		// UserCommands::Favorite { name } => unimplemented!(),
		// UserCommands::Interest { name } => unimplemented!(),
		// UserCommands::Playlist { name, track } => unimplemented!(),
		// UserCommands::Finish { name } => unimplemented!(),
		_ => panic!(),
	};
	// ExitCode::SUCCESS
}
