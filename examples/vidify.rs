//! This example is the configuration file used for Vidify. The config file
//! resides at "vidify/config.ini" inside your user's config directory,
//! but it can also be specified with `--config-file`.
//!
//! It includes custom types with `FromStr` and `ToString`, which are
//! automatically derived with the crate `strum`.

use strum_macros::{Display, EnumString};

#[derive(Debug, Display, EnumString)]
pub enum API {
    MPRIS,
    Windows,
    MacOS,
    SpotifyWeb,
}

#[derive(Debug, Display, EnumString)]
pub enum Player {
    Mpv,
    External,
}

use clap::App;
use structconf::{StructConf, Error};

#[derive(Debug, StructConf)]
pub struct Config {
    #[conf(help = "display debug messages.")]
    debug: bool,
    #[conf(no_file, help = "the config file path.")]
    conf_file: Option<String>,
    #[conf(inverse_arg, default = "true", long = "no_lyrics", short = "n",
           help = "do not print lyrics.")]
    lyrics: bool,
    #[conf(default = "true", help = "open the app in fullscreen mode.")]
    fullscreen: bool,
    #[conf(no_short, help = "activate the dark mode.")]
    dark_mode: bool,
    #[conf(no_short, help = "the window will stay on top of all apps.")]
    stay_on_top: bool,
    #[conf(help = "the source music player used. Read the installation guide \
           for a list with the available APIs.")]
    api: Option<API>,
    #[conf(help = "the output video player. Read the installation guide for \
           a list with the available players.")]
    player: Option<Player>,
    #[conf(no_short, help = "enable automatic audio synchronization. Read \
           the installation guide for more information. Note: this \
           feature is still in development.")]
    audiosync: bool,
    #[conf(no_short, help = "manual tweaking value for audiosync in \
           milliseconds.")]
    audiosync_calibration: i32,
    #[conf(no_short, help = "custom boolean flags used when opening mpv, \
           with dashes and separated by spaces.")]
    mpv_flags: String,
    #[conf(no_short, help = "the client ID for the Spotify Web API. Check \
           the guide to learn how to obtain yours.", section = "SpotifyWeb")]
    client_id: Option<String>,
    #[conf(no_short, help = "the client secret for the Spotify Web API. \
           Check the install guide to learn how to obtain yours.",
           section = "SpotifyWeb")]
    client_secret: Option<String>,
    #[conf(no_short, help = "the redirect URI used for the Spotify Web API.",
           section = "SpotifyWeb")]
    redirect_uri: String,
    #[conf(no_short, no_long, section = "SpotifyWeb")]
    refresh_token: Option<String>,
}

/// Initializes the application's configuration structure. The config file
/// will be at the user's default config path, or whichever is specified
/// by `--config-file`.
fn init_config() -> Result<Config, Error> {
    let app = App::new("vidify")
        .version(clap::crate_version!())
        .author(clap::crate_authors!());
    let args = Config::parse_args(app);
    match args.clone().value_of("config_path") {
        Some(path) => {
            Config::parse_file(args, path)
        },
        None => {
            let mut path = dirs::config_dir()
                .expect("Couldn't find user's config path");
            path.extend(["vidify", "config.ini"].iter());
            Config::parse_file(args, path.to_str().unwrap())
        }
    }
}

pub fn main() {
    let config = init_config();
    println!("Config options: {:#?}", config);
}
