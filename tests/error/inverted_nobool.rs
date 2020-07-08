//! An inverted argument doesn't make sense for something that doesn't
//! implement `std::ops::Not`.

use structconf::StructConf;

#[derive(StructConf)]
struct Config {
    #[conf(arg_inverted)]
    pub value: i32,
}

fn main() {}
