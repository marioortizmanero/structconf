//! This very simple test should parse correctly.

use structconf::StructConf;

#[derive(Debug, StructConf)]
pub struct Config {
    pub debug: bool,
    pub lyrics: bool,
    pub fullscreen: bool,
    pub dark_mode: bool,
}

fn main() {
    let conf = Config::new();
    println!("Debug: {}", conf.read().debug);
    conf.write().debug = true;
    println!("Debug: {}", conf.read().debug);
    conf.write().debug = false;
    println!("Debug: {}", conf.read().debug);
}
