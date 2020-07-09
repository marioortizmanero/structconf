//! An option that isn't available in the config file nor in the argument
//! parser should compile.

use structconf::StructConf;

#[derive(StructConf)]
struct Config {
    #[conf(default = "123")]
    int1: i32,
    #[conf(default = "1.3")]
    float1: f64,
    #[conf(default = "true")]
    bool1: bool,
    #[conf(default = "String::from(\"name\")")]
    str1: String,
}

fn main() {}
