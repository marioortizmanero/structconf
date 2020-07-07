//! The most basic test possible

use structconf::StructConf;

#[derive(StructConf)]
pub struct Config {
    value: bool
}

fn main() {
    let conf = Config::new();
    println!("value: {}", conf.read().unwrap().value);
}
