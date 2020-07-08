//! Includes fields with custom types that implement `ToString`, `FromStr`
//! and `Default`.

use std::str::FromStr;
use std::default::Default;
use std::fmt;
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

struct MyStruct {
    data: i32,
    moredata: String
}

impl FromStr for MyStruct {
    type Err = fmt::Error;

    fn from_str(_s: &str) -> Result<Self, Self::Err> {
        Ok(MyStruct{ data: 123, moredata: String::from("...") })
    }
}

impl fmt::Display for MyStruct {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "...")
    }
}

impl Default for MyStruct {
    fn default() -> Self {
        MyStruct { data: 0, moredata: String::from("") }
    }
}

#[derive(StructConf)]
struct Config {
    e: MyEnum,
    s: MyStruct,
}

fn main() {}
