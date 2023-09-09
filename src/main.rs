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
		UserCommands::Whomst => actions::whomst(),
		UserCommands::Playlist { name, track } => actions::playlist(name, track),
		UserCommands::Consent => actions::consent(),
		UserCommands::NoMusic => actions::no_music(),
		UserCommands::Touch { track } => actions::touch(track),
		UserCommands::Tinish => actions::tinish(),
		UserCommands::Interest { name, timestamp } => actions::interest(name, timestamp),
		UserCommands::Discover { name, timestamp } => actions::discover(name, timestamp),
		UserCommands::Favorite { name, timestamp } => actions::favorite(name, timestamp),
		UserCommands::Unfavorite { name } => actions::unfavorite(name),
		UserCommands::Shark => actions::shark(),
		UserCommands::Rotate { track } => actions::rotate(track),
		UserCommands::Finish => actions::finish(),
		UserCommands::Uninterest => actions::uninterest(),
	}
}
