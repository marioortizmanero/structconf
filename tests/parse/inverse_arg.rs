//! Includes an inverted argument.

use structconf::StructConf;

#[derive(StructConf)]
struct Config {
    #[conf(short = "n", long = "no_thing", inverse_arg)]
    thing: bool,
}

fn main() {}
