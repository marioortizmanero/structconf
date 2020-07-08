//! The most basic test possible

use structconf::StructConf;

#[derive(StructConf)]
pub struct Config {
    value: bool,
}

fn main() {
    let app = clap::App::new("test");
    match Config::parse(app, "test/config.ini") {
        Err(e) => eprintln!("Error when parsing: {}", e),
        Ok(conf) => println!("Config `value` field: {}", conf.value)
    };
}
