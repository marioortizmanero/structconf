//! Includes fields with custom types that implement `ToString`, `FromStr`
//! and `Default`.

use std::default::Default;
use std::fmt;
use std::fs::File;
use std::io::Write;
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

#[test]
fn custom() {
    // Making sure the default values work.
    let mut f = File::create("custom.ini").unwrap();
    f.write(b"
    [Defaults]
    ").unwrap();

    let app = clap::App::new("test");
    let conf = Config::parse(app, "custom.ini").unwrap();
    assert_eq!(conf.someenum, MyEnum::One);
    assert_eq!(conf.astruct, MyStruct {
        data: 0,
        moredata: String::from("(nothing)")
    });

    // With actual values
    f.write(b"
    someenum = Two
    astruct = 123;strval
    ").unwrap();

    let app = clap::App::new("test");
    let conf = Config::parse(app, "custom.ini").unwrap();
    assert_eq!(conf.someenum, MyEnum::Two);
    assert_eq!(conf.astruct, MyStruct {
        data: 123,
        moredata: String::from("strval"),
    });

    // Writing values into the file
    let app = clap::App::new("test");
    let args = Config::parse_args(app);
    let mut conf = Config::parse_file(&args, "custom.ini").unwrap();
    let written_enum = MyEnum::Three;
    let written_struct = MyStruct {
        data: 999,
        moredata: String::from("another value"),
    };
    conf.someenum = written_enum.clone();
    conf.astruct = written_struct.clone();
    conf.write_file("custom.ini").unwrap();

    // Reading it again to make sure the values were written correctly.
    let conf = Config::parse_file(&args, "custom.ini").unwrap();
    assert_eq!(conf.someenum, written_enum);
    assert_eq!(conf.astruct, written_struct);
}
