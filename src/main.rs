#![allow(unused_variables)]

use args::Args;
use args::UserCommands;
use clap::Parser;
use std::process::ExitCode;

mod actions;
mod args;
mod extra;
mod models;
mod sh;

fn main() -> ExitCode {
	let args = Args::parse();
	match args.action {
		UserCommands::Whomst => unimplemented!(),
		UserCommands::Playlist { name, track } => unimplemented!(),
		UserCommands::Consent => actions::consent(),
		UserCommands::Touch { track } => actions::touch(track),
		UserCommands::Tinish => actions::tinish(),
		UserCommands::Interest { name, timestamp } => {
			actions::interest(name, timestamp)
		}
		UserCommands::Discover { name, timestamp } => actions::discover(name, timestamp),
		UserCommands::Favorite { name, timestamp } => unimplemented!(),
		UserCommands::Unfavorite { name } => unimplemented!(),
		UserCommands::Shark => actions::shark(),
		UserCommands::Rotate { track } => actions::rotate(track),
		UserCommands::Finish => actions::finish(),
		UserCommands::Uninterest => actions::uninterest(),
	}
}
