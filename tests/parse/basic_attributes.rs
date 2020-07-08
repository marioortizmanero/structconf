//! Introduces the basic attributes to the tests.

use structconf::StructConf;

#[derive(StructConf)]
struct Config {
    #[conf(no_file)]
    no_file: i32,
    #[conf(no_short)]
    no_short: bool,
    #[conf(no_long)]
    no_long: bool,
    #[conf(no_short, no_long)]
    no_short_no_long: bool,
    #[conf(file = "name")]
    file: bool,
    #[conf(long = "name")]
    long: bool,
    #[conf(short = "n")]
    short: bool,
    #[conf(file = "name", long = "name", short = "n")]
    combined: bool,
}

fn main() {}
