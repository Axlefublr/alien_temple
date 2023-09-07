use std::process::ExitCode;

use args::Args;
use args::UserCommands;
use clap::Parser;

mod args;
mod actions;
mod sh;

fn main() -> ExitCode {
	let args = Args::parse();
	match args.action {
		UserCommands::Consent => unimplemented!(),
		UserCommands::Discover { name } => unimplemented!(),
		UserCommands::Favorite { name } => unimplemented!(),
		UserCommands::Interest { name } => unimplemented!(),
		UserCommands::Playlist { name, track } => unimplemented!(),
		UserCommands::Rotate => unimplemented!(),
		UserCommands::Shark => unimplemented!(),
		UserCommands::Touch => unimplemented!(),
		UserCommands::Whomst => unimplemented!(),
	};
	// ExitCode::SUCCESS
}