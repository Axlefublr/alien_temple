use clap::Parser;
use clap::Subcommand;

#[derive(Parser)]
#[command(author, version, about)]
pub struct Args {
	#[command(subcommand)]
	pub action: UserCommands,
}

#[derive(Subcommand)]
pub enum UserCommands {
	#[command(visible_alias = "who")]
	Whomst,
	#[command(visible_alias = "play")]
	Playlist {
		name: String,
		track: String,
	},
	#[command(visible_alias = "con")]
	Consent,
	NoMusic,
	Touch {
		track: String,
	},
	Tinish,
	#[command(visible_alias = "int")]
	Interest {
		name: String,
		#[arg(short, long)]
		timestamp: Option<String>,
	},
	#[command(visible_alias = "disc")]
	Discover {
		name: String,
		#[arg(short, long)]
		timestamp: Option<String>,
	},
	#[command(visible_alias = "fav")]
	Favorite {
		name: String,
		#[arg(short, long)]
		timestamp: Option<String>,
	},
	#[command(visible_alias = "unfav")]
	Unfavorite {
		name: String,
	},
	Shark,
	Rotate {
		track: String,
	},
	#[command(visible_alias = "fin")]
	Finish,
	#[command(visible_alias = "uni")]
	Uninterest,
}
