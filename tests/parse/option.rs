//! Includes optional fields, that should default to None.

use structconf::StructConf;

#[derive(Display, EnumString)]
enum MyDataType {
    A,
    B,
    C,
}

#[derive(Debug, StructConf)]
struct Config {
    j: Option<i32>,
    b: Option<u64>,
    c: Option<&str>,
    d: Option<MyDataType>,
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
