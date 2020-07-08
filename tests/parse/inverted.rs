//! Includes an inverted argument.

use structconf::StructConf;

#[derive(StructConf)]
struct Config {
    #[conf(short = "n", long = "no_thing", arg_inverted = "true")]
    thing: bool,
}

fn main() {}
