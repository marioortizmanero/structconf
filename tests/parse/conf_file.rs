//! Introduces the conf_file attribute

use structconf::StructConf;

#[derive(StructConf)]
struct Config {
    #[conf(conf_file, help = "specify a config file")]
    config_file: String,
    // Although `no_file` isn't strictly necessary, this should compile.
    #[conf(conf_file, no_file)]
    config_file: String,
    // Custom names can still be applied.
    #[conf(conf_file, long = "conf_file", short = "c")]
    config_file: String,
}

fn main() {}
