//! This very simple test should parse correctly.

use structconf::StructConf;

#[derive(StructConf)]
struct Config {
    #[conf(help = "debug mode", default = "true", section = "thing")]
    pub debug: bool,
    #[conf(help = "value does things", default = true)]
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
