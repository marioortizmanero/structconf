//! An inverted argument doesn't make sense for something that doesn't
//! implement `std::ops::Not`.

use structconf::StructConf;

#[derive(StructConf)]
struct Config {
    #[conf(negated_arg)]
    pub value: f64,
}

fn main() {}
