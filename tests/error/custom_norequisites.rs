//! A struct that doesn't implement `Display` and `FromStr` can't be parsed.
//! `Default` is needed, too.

use structconf::StructConf;

enum MyEnum {
    One,
    Two,
    Three,
}

struct MyStruct {
    data: i32,
    moredata: String
}

#[derive(StructConf)]
struct Config {
    e: MyEnum,
    s: MyStruct,
}

fn main() {}
