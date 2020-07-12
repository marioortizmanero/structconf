//! Shouldn't compile because an identifier is repeated

use structconf::StructConf;

#[derive(StructConf)]
struct Config {
    // Automatically generated short names also count.
    #[conf(no_file, no_long)]
    pub name1: bool,
    #[conf(no_file, no_long)]
    pub name2: bool,
}

fn main() {}
