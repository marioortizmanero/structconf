//! Conflicting options for the help, which should only be available for
//! arguments.

use structconf::StructConf;

#[derive(StructConf)]
struct Config {
    #[conf(no_short, no_long, help = "out of place help message")]
    pub value: bool,
}

fn main() {}
