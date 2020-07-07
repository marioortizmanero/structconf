//! This test should parse correctly. It also contains attributes for the
//! struct, and some real-world use cases.

use structconf::StructConf;

#[derive(Debug, StructConf)]
pub struct Config {
    #[conf(file, long, help = "thing")]
    pub debug: bool,
    #[conf(long, help = "thing")]
    pub config_file: String,
    #[conf(short = "n", long = "no-lyrics", arg_inverted = "true",
           help = "thing", default = "true")]
    pub lyrics: bool,
    #[conf(help = "thing")]
    pub fullscreen: bool,
    #[conf(no_short, help = "thing")]
    pub dark_mode: bool,
    #[conf(no_short, help = "thing")]
    pub stay_on_top: bool,
    // pub api: Option<API>,
    // pub player: Option<Player>,
    #[conf(no_short, help = "thing")]
    pub audiosync: bool,
    #[conf(no_short, help = "thing")]
    pub audiosync_calibration: i32,
    #[conf(no_short, help = "thing")]
    pub mpv_flags: String,
    #[conf(no_short, help = "thing")]
    pub client_id: Option<String>,
    #[conf(no_short, help = "thing", section = "SpotifyWeb")]
    pub client_secret: Option<String>,
    #[conf(no_short, help = "thing", section = "SpotifyWeb")]
    pub redirect_uri: String,
    #[conf(no_short, help = "thing", section = "SpotifyWeb")]
    pub refresh_token: Option<String>
}

fn main() {
    let conf = Config::new();
    println!("Debug: {}", conf.read().unwrap().debug);
    conf.write().unwrap().debug = true;
    println!("Debug: {}", conf.read().unwrap().debug);
    conf.write().unwrap().debug = false;
    println!("Debug: {}", conf.read().unwrap().debug);
}
