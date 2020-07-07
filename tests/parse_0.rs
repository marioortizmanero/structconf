//! The most basic test possible

use structconf::StructConf;

#[derive(StructConf)]
pub struct Config {
    _value: bool
}

fn main() {
    let _conf = Config::new();
}
