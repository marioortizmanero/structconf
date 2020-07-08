//! The most basic test possible.

use structconf::StructConf;

#[derive(StructConf)]
struct Config {
    nothing: bool,
}

fn main() {}
