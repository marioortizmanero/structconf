//! The short name here is a string, not a character.

use structconf::StructConf;

#[derive(StructConf)]
struct Config {
    #[conf(short = "more_than_a_character")]
    pub value: bool,
}

fn main() {}
