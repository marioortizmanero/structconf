//! Includes fields with custom types that implement `ToString`, `FromStr`
//! and `Default`.

use std::default::Default;
use std::fmt;
use std::str::FromStr;
use structconf::StructConf;
use strum_macros::{Display, EnumString};

// The `strum` crate takes care of the automatic conversion to and from
// a string.
#[derive(Debug, Clone, PartialEq, Display, EnumString)]
enum MyEnum {
    One,
    Two,
    Three,
}

impl Default for MyEnum {
    fn default() -> Self {
        MyEnum::One
    }
}

#[derive(Debug, Clone, PartialEq)]
struct MyStruct {
    data: i32,
    moredata: String,
}

impl FromStr for MyStruct {
    type Err = std::num::ParseIntError;

    // Very quick implementation for converting from strings.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut s = s.split(";");
        Ok(MyStruct {
            data: s.next().unwrap().parse::<i32>()?,
            moredata: String::from(s.next().unwrap()),
        })
    }
}

impl fmt::Display for MyStruct {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{};{}", self.data, self.moredata)
    }
}

impl Default for MyStruct {
    fn default() -> Self {
        MyStruct {
            data: 0,
            moredata: String::from("(nothing)"),
        }
    }
}

#[derive(StructConf)]
struct Config {
    someenum: MyEnum,
    astruct: MyStruct,
}

fn main() {}
