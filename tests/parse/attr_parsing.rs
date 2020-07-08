//! Multiple valid ways to parse the attributes.

use structconf::StructConf;

#[derive(StructConf)]
struct Config {
    #[conf(no_short)]
    flag: bool,
    #[conf(no_short = "true")]
    val_str: bool,
    #[conf(no_short = true)]
    val_tok: bool,
}

fn main() {}
