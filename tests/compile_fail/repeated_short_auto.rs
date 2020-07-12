//! Shouldn't compile because an identifier is repeated

use structconf::StructConf;

#[derive(StructConf)]
struct Config {
    #[conf(short = "x")]
    pub a: bool,
    #[conf(short = "x")]
    pub b: bool,
}

fn main() {}
