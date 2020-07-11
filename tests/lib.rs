//! Testing the overall functionality of StructConf. Some of the tests may
//! create new config files, which will appear in the
//! `target/tests/structconf` directory. These files should have different
//! names to avoid conflicts.

use std::default::Default;
use std::fmt;
use std::fs::{self, File};
use std::io::Write;
use structconf::{StructConf, Error};
use std::str::FromStr;
use strum_macros::{Display, EnumString};

/// Handy macro to make sure the test files are cleaned between runs.
macro_rules! setup_file {
    ($name:expr) => {
        fs::remove_file($name).unwrap();
    }
}

/// Defining an enum to use in a StructConf-derived structure.
/// The `strum` crate takes care of the automatic conversion to and from
/// a string.
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

/// Defining a struct to use in a StructConf-derived structure.
/// The required traits are implemented manually.
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

#[derive(Debug, PartialEq, StructConf)]
struct Config {
    #[conf(no_file)]
    no_file: i32,
    #[conf(no_short)]
    no_short: bool,
    #[conf(no_long, short = "x")]
    no_long: bool,
    #[conf(no_short, no_long)]
    no_short_no_long: bool,
    #[conf(no_short, no_long, no_file)]
    empty: i32,
    #[conf(file = "new_file")]
    file: bool,
    #[conf(long = "name")]
    long: bool,
    #[conf(short = "s")]
    short: bool,
    #[conf(file = "new_combined", long = "new_combined", short = "c")]
    combined: bool,
    #[conf(no_short)]
    someenum: MyEnum,
    #[conf(no_short)]
    astruct: MyStruct,
    #[conf(no_short)]
    option_i32: Option<i32>,
    #[conf(no_short)]
    option_enum: Option<MyEnum>,
    #[conf(no_short)]
    option_string: Option<String>,
}

/// Reading values from the configuration file
#[test]
fn file_read() {
    const FILE_NAME: &str = "file_read.ini";
    setup_file!(FILE_NAME);

    let mut f = File::create(FILE_NAME).unwrap();
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
    let conf = Config::parse(app, FILE_NAME).unwrap();
    assert_eq!(conf.no_file, 0);
    assert_eq!(conf.no_short, true);
    assert_eq!(conf.no_long, true);
    assert_eq!(conf.no_short_no_long, false);
    assert_eq!(conf.file, true);
    assert_eq!(conf.long, false);
    assert_eq!(conf.short, false);
    assert_eq!(conf.combined, true);
}

/// Testing the errors that may be thrown when parsing the config
#[test]
fn errors() {
    const FILE_NAME: &str = "errors.ini";
    setup_file!(FILE_NAME);

    // Checking errors when parsing the config file
    let mut f = File::create(FILE_NAME).unwrap();
    f.write(b"
    [Defaults]
    no_short = \"should be a boolean\"
    ").unwrap();

    let app = clap::App::new("test");
    match Config::parse(app, FILE_NAME) {
        Err(Error::Parse(_)) => assert!(true),
        s => assert!(false, "parse error not returned: {:?}", s)
    }

    // Checking errors when creating the config file. The root directory
    // should throw an IO error because it's a directory, and possibly
    // because of invalid permissions.
    let app = clap::App::new("test");
    match Config::parse(app, "/") {
        Err(Error::IO(_)) => assert!(true),
        _ => assert!(false, "IO error not returned")
    }
}

/// Testing parsing for custom structures
#[test]
fn custom_types() {
    const FILE_NAME: &str = "custom_types.ini";
    setup_file!(FILE_NAME);

    // Making sure the default values work.
    let mut f = File::create(FILE_NAME).unwrap();
    f.write(b"
    [Defaults]
    ").unwrap();

    let app = clap::App::new("test");
    let conf = Config::parse(app, FILE_NAME).unwrap();
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
    let conf = Config::parse(app, FILE_NAME).unwrap();
    assert_eq!(conf.someenum, MyEnum::Two);
    assert_eq!(conf.astruct, MyStruct {
        data: 123,
        moredata: String::from("strval"),
    });
}

#[test]
fn write_file() {
    const FILE_NAME: &str = "write_file.ini";
    setup_file!(FILE_NAME);

    // Writing values into the file
    let app = clap::App::new("test");
    let args = Config::parse_args(app);
    let mut conf = Config::parse_file(&args, FILE_NAME).unwrap();
    let written_enum = MyEnum::Three;
    let written_struct = MyStruct {
        data: 999,
        moredata: String::from("another value"),
    };
    conf.someenum = written_enum.clone();
    conf.astruct = written_struct.clone();
    conf.write_file(FILE_NAME).unwrap();

    // Reading it again to make sure the values were written correctly.
    let conf = Config::parse_file(&args, FILE_NAME).unwrap();
    assert_eq!(conf.someenum, written_enum);
    assert_eq!(conf.astruct, written_struct);
}

#[test]
fn inverse_arg() {
    // TODO
}

/// Making sure an empty field isn't included in the config file or the
/// argument parser.
#[test]
fn empty_field() {
    const FILE_NAME: &str = "empty_field.ini";
    setup_file!(FILE_NAME);

    let mut f = File::create(FILE_NAME).unwrap();
    f.write(b"
    [Defaults]
    empty = 1234
    ").unwrap();

    let app = clap::App::new("test");
    let args = Config::parse_args(app);
    let conf = Config::parse_file(&args, FILE_NAME).unwrap();
    // Should take the default value after parsing the config file and the
    // arguments.
    assert_eq!(conf.empty, 0);
}

// Options in `Option<T>` should be handled as expected.
#[test]
fn optionals() {
    const FILE_NAME: &str = "optionals.ini";
    setup_file!(FILE_NAME);

    // First all of them should be None because the file is empty
    let app = clap::App::new("test");
    let conf = Config::parse(app, FILE_NAME).unwrap();
    assert_eq!(conf.option_i32, None);
    assert_eq!(conf.option_enum, None);
    assert_eq!(conf.option_string, None);

    // Some values are writte into the config file and it's parsed again.
    let mut f = File::create(FILE_NAME).unwrap();
    f.write(b"
    [Defaults]
    option_i32 = 1234
    option_enum = Three
    option_string = some text goes here
    ").unwrap();

    // The new values should appear under `Some`.
    let app = clap::App::new("test");
    let conf = Config::parse(app, FILE_NAME).unwrap();
    assert_eq!(conf.option_i32, Some(1234));
    assert_eq!(conf.option_enum, Some(MyEnum::Three));
    assert_eq!(conf.option_string, Some(String::from("some text goes here")));

    // Writing some optional values. Only those that aren't `None` should
    // be written into it.
    let app = clap::App::new("test");
    let mut conf = Config::parse(app, FILE_NAME).unwrap();
    let written_i32 = None;
    let written_enum = Some(MyEnum::Two);
    let written_string = Some(String::from("value"));

    conf.option_i32 = written_i32;
    conf.option_enum = written_enum.clone();
    conf.option_string = written_string.clone();
    fs::remove_file(FILE_NAME).unwrap();
    conf.write_file(FILE_NAME).unwrap();

    let app = clap::App::new("test");
    let conf = Config::parse(app, FILE_NAME).unwrap();
    assert_eq!(conf.option_i32, written_i32);
    assert_eq!(conf.option_enum, written_enum);
    assert_eq!(conf.option_string, written_string);
}
