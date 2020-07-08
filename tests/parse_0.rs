//! The most basic test possible

use structconf::StructConf;

#[derive(StructConf)]
pub struct Config {
    value: bool,
}

fn main() {
    let app = clap::App::new("test");
    let conf = Config::parse(app, "test/config.ini");
    println!("value: {}", conf.value);
}
