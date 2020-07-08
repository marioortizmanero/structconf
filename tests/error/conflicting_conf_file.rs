//! The conf_file option means it's not available as a config file option.

use structconf::StructConf;

#[derive(StructConf)]
struct Config {
    #[conf(conf_file, file = "illegal")]
    pub value: bool,
}

fn main() {}
