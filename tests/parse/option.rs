//! Includes optional fields, that should default to None.

use std::fmt;
use std::str::FromStr;
use std::default::Default;
use structconf::StructConf;

enum MyEnum {
    One,
    Two,
    Three,
}

impl FromStr for MyEnum {
    type Err = fmt::Error;

    fn from_str(_s: &str) -> Result<Self, Self::Err> {
        Ok(MyEnum::One)
    }
}

impl fmt::Display for MyEnum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "...")
    }
}

impl Default for MyEnum {
    fn default() -> Self {
        MyEnum::One
    }
}

#[derive(StructConf)]
struct Config {
    a: Option<i32>,
    b: Option<f64>,
    d: Option<MyEnum>,
    e: Option<String>,
    #[conf(no_short)]
    f: Option<String>,
    #[conf(no_long)]
    g: Option<String>,
    #[conf(no_short, no_long)]
    h: Option<String>,
    #[conf(no_file)]
    i: Option<String>,
    #[conf(no_short, no_long, no_file)]
    j: Option<String>,
}

fn main() {}
