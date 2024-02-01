use clap::Parser;
use clap::Subcommand;

#[derive(Parser)]
#[command(author, version)]
pub struct Args {
    #[command(subcommand)]
    pub action: UserCommands,
}

#[derive(Subcommand)]
pub enum UserCommands {
    /// Last playlisted track
    #[command(visible_alias = "who")]
    Whomst,
    /// Next playlist to add to
    Next,
    /// Update last playlisted track
    #[command(visible_alias = "play")]
    Playlist {
        name:  String,
        track: String,
    },
    /// Who should I touch next?
    #[command(visible_alias = "con")]
    Consent,
    /// Person I wanted to touch doesn't have any music
    Nomusic,
    /// Touch the topmost artist, moving them to rotation
    Touch {
        track: String,
    },
    /// Touch and finish
    Tinish,
    /// Add artist to new list
    #[command(visible_alias = "int")]
    Interest {
        name:      String,
        #[arg(short, long)]
        timestamp: Option<String>,
    },
    #[command(visible_alias = "disc")]
    Discover {
        name:      String,
        #[arg(short, long)]
        timestamp: Option<String>,
    },
    #[command(visible_alias = "fav")]
    Favorite {
        name:      String,
        #[arg(short, long)]
        timestamp: Option<String>,
    },
    #[command(visible_alias = "unfav")]
    Unfavorite {
        name: String,
    },
    /// Who should I rotate next?
    Shark,
    Rotate {
        track: String,
    },
    #[command(visible_alias = "fin")]
    Finish,
    /// Remove person from rotation
    #[command(visible_alias = "uni")]
    Uninterest {
        name: Option<String>,
    },
}
