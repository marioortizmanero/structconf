//! Includes an inverted argument.

use structconf::StructConf;

#[allow(dead_code)]
#[derive(StructConf)]
struct Config {
    #[conf(negated_arg, short = "n", long = "no_thing")]
    thing: bool,
}

fn main() {}
