//! Shouldn't compile because an identifier is repeated

use structconf::StructConf;

#[derive(StructConf)]
struct Config {
    // Automatically generated short names also count.
    pub name1: bool,
    pub name2: bool,
}

fn main() {}
