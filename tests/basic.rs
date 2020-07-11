//! The most basic test possible.

use structconf::StructConf;

#[allow(dead_code)]
#[derive(StructConf)]
struct Config {
    nothing: bool,
}

#[test]
fn basic() {}
