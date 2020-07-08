//! Conflicting options for the long argument.

use structconf::StructConf;

#[derive(StructConf)]
struct Config {
    #[conf(no_long, long = "thing")]
    pub value: bool,
}

fn main() {}
