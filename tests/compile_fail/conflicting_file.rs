//! Conflicting options for the file name.

use structconf::StructConf;

#[derive(StructConf)]
struct Config {
    #[conf(no_file, file = "thing")]
    pub value: bool,
}

fn main() {}
