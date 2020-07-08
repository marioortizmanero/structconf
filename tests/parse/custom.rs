//! Includes fields with custom types that implement ToString and FromStr.

use structconf::StructConf;

#[derive(Display, EnumString)]
enum MyEnum {
    One,
    Two,
    Three,
}

#[derive(Display, EnumString)]
struct MyStruct {
    data: i32,
    moredata: String
}

#[derive(Debug, StructConf)]
struct Config {
    d: MyDataType,
    d: MyStruct,
}

fn main() {}
