//! Shouldn't compile because an identifier is repeated

use structconf::StructConf;

#[derive(StructConf)]
struct Config {
    #[conf(file = "abc")]
    pub name1: bool,
    #[conf(file = "abc")]
    pub name2: bool,
}

fn main() {}
