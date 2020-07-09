//! Conflicting options for inverted arguments, which don't make sense for
//! config-file-only options.

use structconf::StructConf;

#[derive(StructConf)]
struct Config {
    #[conf(no_short, no_long, inverted_arg)]
    pub value: bool,
}

fn main() {}
