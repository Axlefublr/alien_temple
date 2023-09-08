#![allow(unused_variables)]

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
		UserCommands::Interest { name, timestamp } => actions::interest(name, timestamp, should_commit),
		UserCommands::Shark => actions::shark(),
		UserCommands::Rotate { track } => actions::rotate(track, should_commit),
		UserCommands::Finish => actions::finish(should_commit),
		UserCommands::Uninterest { name } => unimplemented!(),
		UserCommands::Discover { name } => unimplemented!(),
		UserCommands::Favorite { name } => unimplemented!(),
		UserCommands::Unfavorite { name } => unimplemented!(),
		UserCommands::Whomst => unimplemented!(),
		UserCommands::Playlist { name, track } => unimplemented!(),
	}
}
