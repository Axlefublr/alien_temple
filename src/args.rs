use clap::Parser;
use clap::Subcommand;

#[derive(Parser)]
#[command(author, version, about)]
pub struct Args {
	#[command(subcommand)]
	pub action: UserCommands,
	#[arg(short, long)]
	pub git: bool
}

#[derive(Subcommand)]
pub enum UserCommands {
	#[command(visible_alias = "who")]
	Whomst,
	#[command(visible_alias = "play")]
	Playlist {
		name: String,
		track: String
	},
	#[command(visible_alias = "disc")]
	Discover {
		name: String
	},
	#[command(visible_alias = "int")]
	Interest {
		name: String
	},
	#[command(visible_alias = "fav")]
	Favorite {
		name: String
	},
	#[command(visible_alias = "con")]
	Consent,
	Touch,
	Shark,
	Rotate
}