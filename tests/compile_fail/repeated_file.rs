//! Shouldn't compile because an identifier is repeated

use structconf::StructConf;

#[derive(StructConf)]
struct Config {
    #[conf(no_short, no_long, file = "abc")]
    pub name1: bool,
    #[conf(no_short, no_long, file = "abc")]
    pub name2: bool,
}

fn main() {}
