//! Multiple valid ways to parse the attributes.

use structconf::StructConf;

#[allow(dead_code)]
#[derive(StructConf)]
struct Config {
    #[conf(no_short)]
    flag: bool,
    #[conf(no_short = "true")]
    val_str: bool,
    #[conf(no_short = true)]
    val_tok: bool,
}

#[test]
fn attr_parsing() {}
