//! Testing the overall functionality of StructConf. Some of the tests may
//! create new config files, which will appear in the
//! `target/tests/structconf` directory. These files should have different
//! names to avoid conflicts.

use std::convert::AsRef;
use std::default::Default;
use std::fmt;
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;
use std::str::FromStr;
use structconf::{Error, StructConf};
use strum_macros::{Display, EnumString};

/// `TempFile` is a very simple wrapper for automatically cleaning up files
/// used in the tests.
struct TempFile(String);

impl TempFile {
    pub fn new(path: &str) -> TempFile {
        fs::remove_file(path).ok();
        TempFile(path.to_string())
    }
}

impl Drop for TempFile {
    fn drop(&mut self) {
        fs::remove_file(&self.0).ok();
    }
}

impl AsRef<str> for TempFile {
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}

impl AsRef<Path> for TempFile {
    fn as_ref(&self) -> &Path {
        Path::new(&self.0)
    }
}

impl std::ops::Deref for TempFile {
    type Target = str;

    fn deref(&self) -> &str {
        &self.0
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
    #[conf(long = "new_long")]
    long: bool,
    #[conf(short = "s")]
    short: bool,
    #[conf(file = "new_combined", long = "new_combined", short = "c")]
    combined: bool,
    #[conf(default = "123 + 123")]
    default: i64,
    #[conf(no_short, inverse_arg, long = "no-value", default = "true")]
    inverse: bool,
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
fn read_file() {
    let file = TempFile::new("file_read.ini");

    let mut f = File::create(&file).unwrap();
    // `no_file` should be 0 because this option isn't available in the
    // config file.
    // `no_short_no_long` and others are not included; they should be false.
    // `new_file` and `new_combined` have been renamed from `file` and
    // `combined`, respectively.
    f.write(
        b"[Defaults]
no_file = 1234
no_short = \"true\"
no_long = true
new_file = true
new_combined = true
someenum = Two
astruct = 123;strval
empty = 2134",
    )
    .unwrap();

    let app = clap::App::new("test");
    let conf = Config::parse(app, &file).unwrap();
    assert_eq!(conf.no_file, 0);
    assert_eq!(conf.no_short, true);
    assert_eq!(conf.no_long, true);
    assert_eq!(conf.no_short_no_long, false);
    assert_eq!(conf.file, true);
    assert_eq!(conf.long, false);
    assert_eq!(conf.short, false);
    assert_eq!(conf.combined, true);
    assert_eq!(conf.empty, 0); // not a config file option, should be default
    assert_eq!(conf.someenum, MyEnum::Two);
    assert_eq!(
        conf.astruct,
        MyStruct {
            data: 123,
            moredata: String::from("strval"),
        }
    );
}

#[test]
fn write_file() {
    let file = TempFile::new("write_file.ini");

    // Writing values into the file
    let app = clap::App::new("test");
    let args = Config::parse_args(app);
    let mut conf = Config::parse_file(&args, &file).unwrap();
    let written_enum = MyEnum::Three;
    let written_struct = MyStruct {
        data: 999,
        moredata: String::from("another value"),
    };
    conf.someenum = written_enum.clone();
    conf.astruct = written_struct.clone();
    conf.file = true;
    conf.long = true;
    conf.short = true;
    conf.write_file(&file).unwrap();

    // Reading it again to make sure the values were written correctly.
    let conf = Config::parse_file(&args, &file).unwrap();
    assert_eq!(conf.someenum, written_enum);
    assert_eq!(conf.astruct, written_struct);
    assert_eq!(conf.file, true);
    assert_eq!(conf.long, true);
    assert_eq!(conf.short, true);
    assert_eq!(conf.combined, false); // Not written, it's the default
}

/// Making sure that the basic arguments are generated correctly.
#[test]
fn args() {
    let file = TempFile::new("args.ini");

    let app = clap::App::new("test");
    let args = vec![
        "test", // Program name
        "--no-file",
        "123", // Long for `no_file`
        "--no-short",
        "-x", // Renamed short for `no_long`
        "--file",
        "--new-long", // Renamed long for `long`
        "--option-i32",
        "321",        // Optional parameter
        "--no-value", // Long for `inverse`, should take the opposite value
    ];
    let args = Config::parse_args_from(app, args);
    let conf = Config::parse_file(&args, &file).unwrap();
    assert_eq!(conf.no_file, 123);
    assert_eq!(conf.no_short, true);
    assert_eq!(conf.no_long, true);
    assert_eq!(conf.no_short_no_long, false);
    assert_eq!(conf.empty, 0); // not an argument, should be default
    assert_eq!(conf.file, true);
    assert_eq!(conf.inverse, false);
    assert_eq!(conf.option_i32, Some(321));
}

/// Proving the "arguments > config file > defaults" priority is correct.
#[test]
fn priorities() {
    // TODO
}

/// Testing the errors that may be thrown when parsing the config
#[test]
fn errors() {
    let file = TempFile::new("errors.ini");

    // Checking errors when parsing the config file
    let mut f = File::create(&file).unwrap();
    f.write(
        b"
[Defaults]
no_short = \"should be a boolean\"",
    )
    .unwrap();

    let app = clap::App::new("test");
    match Config::parse(app, &file) {
        Err(Error::Parse(_)) => assert!(true),
        s => assert!(false, "parse error not returned: {:?}", s),
    }

    // Checking errors when creating the config file. The root directory
    // should throw an IO error because it's a directory, and possibly
    // because of invalid permissions.
    let app = clap::App::new("test");
    match Config::parse(app, "/") {
        Err(Error::IO(_)) => assert!(true),
        _ => assert!(false, "IO error not returned"),
    }
}

// Options in `Option<T>` should be handled as expected.
#[test]
fn optionals() {
    let file = TempFile::new("optionals.ini");

    // First all of them should be None because the file is empty
    let app = clap::App::new("test");
    let conf = Config::parse(app, &file).unwrap();
    assert_eq!(conf.option_i32, None);
    assert_eq!(conf.option_enum, None);
    assert_eq!(conf.option_string, None);

    // Some values are writte into the config file and it's parsed again.
    let mut f = File::create(&file).unwrap();
    f.write(
        b"[Defaults]
option_i32 = 1234
option_enum = Three
option_string = some text goes here",
    )
    .unwrap();

    // The new values should appear under `Some`.
    let app = clap::App::new("test");
    let conf = Config::parse(app, &file).unwrap();
    assert_eq!(conf.option_i32, Some(1234));
    assert_eq!(conf.option_enum, Some(MyEnum::Three));
    assert_eq!(
        conf.option_string,
        Some(String::from("some text goes here"))
    );

    // Writing some optional values. Only those that aren't `None` should
    // be written into it.
    let app = clap::App::new("test");
    let mut conf = Config::parse(app, &file).unwrap();
    let written_i32 = None;
    let written_enum = Some(MyEnum::Two);
    let written_string = Some(String::from("value"));

    conf.option_i32 = written_i32;
    conf.option_enum = written_enum.clone();
    conf.option_string = written_string.clone();
    fs::remove_file(&file).unwrap();
    conf.write_file(&file).unwrap();

    let app = clap::App::new("test");
    let conf = Config::parse(app, &file).unwrap();
    assert_eq!(conf.option_i32, written_i32);
    assert_eq!(conf.option_enum, written_enum);
    assert_eq!(conf.option_string, written_string);
}

/// Making sure the default values work in all cases.
#[test]
fn defaults() {
    let file = TempFile::new("custom_types.ini");

    let mut f = File::create(&file).unwrap();
    f.write(b"[Defaults]").unwrap();

    let app = clap::App::new("test");
    let conf = Config::parse(app, &file).unwrap();
    assert_eq!(conf.no_file, 0);
    assert_eq!(conf.no_short, false);
    assert_eq!(conf.no_long, false);
    assert_eq!(conf.no_short_no_long, false);
    assert_eq!(conf.empty, 0);
    assert_eq!(conf.default, 246);
    assert_eq!(conf.option_i32, None);
    assert_eq!(conf.someenum, Default::default());
    assert_eq!(conf.astruct, Default::default());
}
