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
		UserCommands::Whomst => unimplemented!(),
		UserCommands::Playlist { name, track } => unimplemented!(),
		UserCommands::Consent => actions::consent(),
		UserCommands::Touch { track} => actions::touch(track, should_commit),
		UserCommands::Tinish => actions::tinish(should_commit),
		UserCommands::Interest { name, timestamp } => actions::interest(name, timestamp, should_commit),
		UserCommands::Discover { name, timestamp } => unimplemented!(),
		UserCommands::Favorite { name, timestamp } => unimplemented!(),
		UserCommands::Unfavorite { name } => unimplemented!(),
		UserCommands::Shark => actions::shark(),
		UserCommands::Rotate { track } => actions::rotate(track, should_commit),
		UserCommands::Finish => actions::finish(should_commit),
		UserCommands::Uninterest { name } => unimplemented!(),
	}
}
