//! Conflicting options for the short argument.

use structconf::StructConf;

#[derive(StructConf)]
struct Config {
    #[conf(no_short, short = "v")]
    pub value: bool,
}

fn main() {}
