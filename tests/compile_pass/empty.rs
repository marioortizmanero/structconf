//! An option that isn't available in the config file nor in the argument
//! parser should compile.

use structconf::StructConf;

#[allow(dead_code)]
#[derive(StructConf)]
struct Config {
    #[conf(no_file, no_short, no_long)]
    ignore_all: i32,
}

fn main() {}
