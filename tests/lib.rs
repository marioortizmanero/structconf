//! Testing the overall functionality of StructConf.

use std::default::Default;
use std::fmt;
use std::fs::File;
use std::io::Write;
use structconf::{StructConf, Error};
use std::str::FromStr;
use strum_macros::{Display, EnumString};

#[derive(Debug, PartialEq, StructConf)]
struct BasicAttributesConfig {
    #[conf(no_file)]
    no_file: i32,
    #[conf(no_short)]
    no_short: bool,
    #[conf(no_long, short = "x")]
    no_long: bool,
    #[conf(no_short, no_long)]
    no_short_no_long: bool,
    #[conf(file = "new_file")]
    file: bool,
    #[conf(long = "name")]
    long: bool,
    #[conf(short = "s")]
    short: bool,
    #[conf(file = "new_combined", long = "new_combined", short = "c")]
    combined: bool,
}

#[test]
fn basic_attributes() {
    let mut f = File::create("basic_attributes.ini").unwrap();
    // `no_file` should be 0 because this option isn't available in the
    // config file.
    // `no_short_no_long` and others are not included; they should be false.
    // `new_file` and `new_combined` have been renamed from `file` and
    // `combined`, respectively.
    f.write(b"
    [Defaults]
    no_file = 1234
    no_short = \"true\"
    no_long = true
    new_file = true
    new_combined = true
    ").unwrap();

    let app = clap::App::new("test");
    let conf = BasicAttributesConfig::parse(app, "basic_attributes.ini").unwrap();
    assert_eq!(conf.no_file, 0);
    assert_eq!(conf.no_short, true);
    assert_eq!(conf.no_long, true);
    assert_eq!(conf.no_short_no_long, false);
    assert_eq!(conf.file, true);
    assert_eq!(conf.long, false);
    assert_eq!(conf.short, false);
    assert_eq!(conf.combined, true);

    // Checking errors when parsing the config file
    let mut f = File::create("basic_attributes.ini").unwrap();
    f.write(b"
    [Defaults]
    no_short = \"should be a boolean\"
    ").unwrap();

    let app = clap::App::new("test");
    match BasicAttributesConfig::parse(app, "basic_attributes.ini") {
        Err(Error::Parse(_)) => assert!(true),
        s => assert!(false, "parse error not returned: {:?}", s)
    }

    // Checking errors when creating the config file
    let app = clap::App::new("test");
    match BasicAttributesConfig::parse(app, "/") {
        Err(Error::IO(_)) => assert!(true),
        _ => assert!(false, "IO error not returned")
    }
}

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
struct CustomConfig {
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
    let conf = CustomConfig::parse(app, "custom.ini").unwrap();
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
    let conf = CustomConfig::parse(app, "custom.ini").unwrap();
    assert_eq!(conf.someenum, MyEnum::Two);
    assert_eq!(conf.astruct, MyStruct {
        data: 123,
        moredata: String::from("strval"),
    });

    // Writing values into the file
    let app = clap::App::new("test");
    let args = CustomConfig::parse_args(app);
    let mut conf = CustomConfig::parse_file(&args, "custom.ini").unwrap();
    let written_enum = MyEnum::Three;
    let written_struct = MyStruct {
        data: 999,
        moredata: String::from("another value"),
    };
    conf.someenum = written_enum.clone();
    conf.astruct = written_struct.clone();
    conf.write_file("custom.ini").unwrap();

    // Reading it again to make sure the values were written correctly.
    let conf = CustomConfig::parse_file(&args, "custom.ini").unwrap();
    assert_eq!(conf.someenum, written_enum);
    assert_eq!(conf.astruct, written_struct);
}

#[test]
fn inverse_arg() {
    // TODO
}

#[derive(StructConf)]
struct EmptyConfig {
    #[conf(no_file, no_short, no_long)]
    ignore_all: i32,
}

#[test]
fn empty() {
    // TODO
}

#[derive(StructConf)]
struct DefaultConfig {
    #[conf(default = "123")]
    int1: i32,
    #[conf(default = "1.3")]
    float1: f64,
    #[conf(default = "true")]
    bool1: bool,
    #[conf(default = "String::from(\"name\")")]
    str1: String,
}

#[test]
fn default() {
    // TODO
}

#[derive(StructConf)]
struct OptionConfig {
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

#[test]
fn option() {
    // TODO
}
