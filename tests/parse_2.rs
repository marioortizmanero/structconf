//! This test should parse correctly. It also contains attributes for the
//! struct, and some real-world use cases.

use structconf::StructConf;

#[derive(Debug, StructConf)]
pub struct Config {
    #[conf(file, long, help = "thing")]
    pub debug: bool,
    #[conf(long, help = "thing")]
    pub config_file: String,
    #[conf(file, short = "n", long = "no-lyrics", arg_inverted = "true",
           help = "thing", default = "true")]
    pub lyrics: bool,
    #[conf(file, short, long, help = "thing")]
    pub fullscreen: bool,
    #[conf(file, long, help = "thing")]
    pub dark_mode: bool,
    #[conf(file, long, help = "thing")]
    pub stay_on_top: bool,
    // pub api: Option<API>,
    // pub player: Option<Player>,
    #[conf(file, long, help = "thing")]
    pub audiosync: bool,
    #[conf(file, long, help = "thing")]
    pub audiosync_calibration: i32,
    #[conf(file, long, help = "thing")]
    pub mpv_flags: String,
    #[conf(file, long, help = "thing")]
    pub client_id: Option<String>,
    #[conf(file, long, help = "thing", section = "SpotifyWeb")]
    pub client_secret: Option<String>,
    #[conf(file, long, help = "thing", section = "SpotifyWeb")]
    pub redirect_uri: String,
    #[conf(file, long, help = "thing", section = "SpotifyWeb")]
    pub refresh_token: Option<String>
}

fn main() {
    let conf = Config::new();
    println!("Debug: {}", conf.read().debug);
    conf.write().debug = true;
    println!("Debug: {}", conf.read().debug);
    conf.write().debug = false;
    println!("Debug: {}", conf.read().debug);
}
