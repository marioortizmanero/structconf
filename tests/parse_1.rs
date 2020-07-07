//! This very simple test should parse correctly.

use structconf::StructConf;

#[derive(StructConf)]
struct Config {
    #[conf(default = "true", section = "thing")]
    pub debug: bool,
    #[conf(default = true)]
    pub value: bool,
}

fn main() {
    let app = clap::App::new("test");
    let mut conf = Config::new(app);

    println!("Debug: {}", conf.debug);
    conf.debug = true;
    println!("Debug: {}", conf.debug);
    conf.debug = false;
    println!("Debug: {}", conf.debug);
}
