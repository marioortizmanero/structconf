//! This very simple test should parse correctly.

use structconf::StructConf;

#[derive(StructConf)]
struct Config {
    #[conf(section = "thing", default = "true")]
    pub debug: bool,
    #[conf(section = "thing", default = "1234")]
    pub value: bool,
}

fn main() {
    let conf = Config::new();
    println!("Debug: {}", conf.read().unwrap().debug);
    conf.write().unwrap().debug = true;
    println!("Debug: {}", conf.read().unwrap().debug);
    conf.write().unwrap().debug = false;
    println!("Debug: {}", conf.read().unwrap().debug);
}