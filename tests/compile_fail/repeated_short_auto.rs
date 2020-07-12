//! Shouldn't compile because an identifier is repeated

use structconf::StructConf;

#[derive(StructConf)]
struct Config {
    #[conf(no_file, no_long, short = "x")]
    pub name1: bool,
    #[conf(no_file, no_long, short = "x")]
    pub name2: bool,
}

fn main() {}
