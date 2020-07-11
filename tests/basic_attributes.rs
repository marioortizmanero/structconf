use std::fs::File;
use std::io::Write;
use structconf::{StructConf, Error};

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
    let conf = Config::parse(app, "basic_attributes.ini").unwrap();
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
    match Config::parse(app, "basic_attributes.ini") {
        Err(Error::Parse(_)) => assert!(true),
        s => assert!(false, "parse error not returned: {:?}", s)
    }

    // Checking errors when creating the config file
    let app = clap::App::new("test");
    match Config::parse(app, "/") {
        Err(Error::IO(_)) => assert!(true),
        _ => assert!(false, "IO error not returned")
    }
}
