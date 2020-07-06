//! This test should parse correctly. It also contains attributes for the
//! struct, and some real-world use cases.

use structconf::StructConf;

#[derive(Debug, StructConf)]
pub struct Config {
    #[conf(long, help = "thing", section = "Defalts")]
    pub debug: bool,
    #[conf(short = "n", long = "no-lyrics", arg_inverted = "true",
           help = "thing", section = "Defalts", default = "true")]
    pub lyrics: bool,
    #[conf(short, long, help = "thing", section = "Defalts")]
    pub fullscreen: bool,
    #[conf(long, help = "thing", section = "Defalts")]
    pub dark_mode: bool,
    #[conf(long, help = "thing", section = "Defalts")]
    pub stay_on_top: bool,
    // pub api: Option<API>,
    // pub player: Option<Player>,
    #[conf(long, help = "thing", section = "Defalts")]
    pub audiosync: bool,
    #[conf(long, help = "thing", section = "Defalts")]
    pub audiosync_calibration: i32,
    #[conf(long, help = "thing", section = "Defalts")]
    pub mpv_flags: String,
    #[conf(long, help = "thing", section = "Defalts")]
    pub client_id: Option<String>,
    #[conf(long, help = "thing", section = "SpotifyWeb")]
    pub client_secret: Option<String>,
    #[conf(long, help = "thing", section = "SpotifyWeb")]
    pub redirect_uri: String,
    #[conf(long, help = "thing", section = "SpotifyWeb")]
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
