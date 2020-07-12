//! Shouldn't compile because an identifier is repeated

use structconf::StructConf;

#[derive(StructConf)]
struct Config {
    #[conf(no_file, no_short, long = "abc")]
    pub name1: bool,
    #[conf(no_file, no_short, long = "abc")]
    pub name2: bool,
}

fn main() {}
