//! This example is the configuration file that was going to be used for Vidify
//! (https://github.com/vidify/vidify).
//!
//! The config file resides at "vidify/config.ini" inside your user's config
//! directory, but it can also be specified with `--config-file`.
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

use structconf::{clap, Error, StructConf};

#[derive(Debug, StructConf)]
pub struct Config {
    #[conf(help = "Display debug messages")]
    debug: bool,

    #[conf(no_file, help = "The config file path")]
    conf_file: Option<String>,

    #[conf(
        negated_arg,
        long = "no_lyrics",
        short = "n",
        help = "Do not print lyrics"
    )]
    lyrics: bool,

    #[conf(help = "Open the app in fullscreen mode")]
    fullscreen: bool,

    #[conf(no_short, help = "Activate the dark mode")]
    dark_mode: bool,

    #[conf(no_short, help = "The window will stay on top of all apps")]
    stay_on_top: bool,

    #[conf(help = "The source music player used. Read the installation guide \
           for a list with the available APIs")]
    api: Option<API>,

    #[conf(help = "The output video player. Read the installation guide for \
           a list with the available players")]
    player: Option<Player>,

    #[conf(
        no_short,
        help = "Enable automatic audio synchronization. Read the \
           installation guide for more information. Note: this feature is \
           still in development"
    )]
    audiosync: bool,

    #[conf(no_short, help = "Manual tweaking value for audiosync in milliseconds")]
    audiosync_calibration: i32,

    #[conf(
        no_short,
        help = "Custom boolean flags used when opening mpv, with dashes and \
           separated by spaces"
    )]
    mpv_flags: String,

    #[conf(
        no_short,
        help = "The client ID for the Spotify Web API. Check the guide to \
           learn how to obtain yours",
        section = "SpotifyWeb"
    )]
    client_id: Option<String>,

    #[conf(
        no_short,
        help = "The client secret for the Spotify Web API. Check the install \
           guide to learn how to obtain yours",
        section = "SpotifyWeb"
    )]
    client_secret: Option<String>,

    #[conf(
        no_short,
        help = "The redirect URI used for the Spotify Web API",
        section = "SpotifyWeb"
    )]
    redirect_uri: String,

    #[conf(no_short, no_long, section = "SpotifyWeb")]
    refresh_token: Option<String>,
}

/// Initializes the application's configuration structure. The config file
/// will be at the user's default config path, or whichever is specified
/// by `--config-file`.
fn init_config() -> Result<Config, Error> {
    let app = clap::App::new("vidify")
        .version(clap::crate_version!())
        .author(clap::crate_authors!());
    let args = Config::parse_args(app);
    let path = match args.value_of("config_path") {
        Some(path) => path.to_string(),
        None => {
            let mut path = dirs::config_dir().expect("Couldn't find user's config path");
            path.extend(["vidify", "config.ini"].iter());
            path.to_string_lossy().into_owned()
        }
    };
    Config::parse_file(&args, &path)
}

pub fn main() {
    let config = init_config();
    println!("Config options: {:#?}", config);
}
