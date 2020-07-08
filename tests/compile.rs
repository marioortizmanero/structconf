#![feature(box_syntax, test, fmt_internals)]
#![feature(prelude_import)]
//! Introduces the basic attributes to the tests.
#[prelude_import]
use std::prelude::v1::*;
#[macro_use]
extern crate std;
use structconf::StructConf;
struct Config {
    #[conf(no_file)]
    no_file: i32,
    #[conf(no_short)]
    no_short: bool,
    #[conf(no_long)]
    no_long: bool,
    #[conf(no_short, no_long)]
    no_short_no_long: bool,
    #[conf(file = "name")]
    file: bool,
    #[conf(long = "name")]
    long: bool,
    #[conf(short = "n")]
    short: bool,
    #[conf(file = "name", long = "name", short = "n")]
    combined: bool,
}
impl StructConf for Config {
    fn parse(app: ::clap::App, path: &str) -> Result<Config, ::structconf::Error>
    where
        Self: Sized,
    {
        let args = Config::parse_args(app);
        Config::parse_file(args, path)
    }
    fn parse_args<'a>(app: ::clap::App<'a, 'a>) -> ::clap::ArgMatches<'a> {
        ()
    }
    fn parse_file(args: ::clap::ArgMatches, path: &str) -> Result<Config, ::structconf::Error>
    where
        Self: Sized,
    {
        let path_wrap = ::std::path::Path::new(path);
        if !path_wrap.exists() {
            ::std::fs::File::create(&path_wrap)?;
            {
                ::std::io::_print(::core::fmt::Arguments::new_v1(
                    &["Created config file at ", "\n"],
                    &match (&path,) {
                        (arg0,) => [::core::fmt::ArgumentV1::new(
                            arg0,
                            ::core::fmt::Display::fmt,
                        )],
                    },
                ));
            };
        }
        let file = ::ini::Ini::load_from_file(path)?;
        Ok(Config {
            no_file: args
                .value_of("no_file")
                .and_then(|x| {
                    Some(x.parse::<i32>().expect(&{
                        let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                            &[
                                "The value for \'",
                                "\' is invalid in the configuration: \'",
                                "\'",
                            ],
                            &match (&"no_file", &x) {
                                (arg0, arg1) => [
                                    ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Display::fmt),
                                    ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Display::fmt),
                                ],
                            },
                        ));
                        res
                    }))
                })
                .unwrap_or(::std::default::Default::default()),
            no_short: args
                .value_of("no_short")
                .or_else(|| file.get_from(Some("Defaults"), "no_short"))
                .and_then(|x| {
                    Some(x.parse::<bool>().expect(&{
                        let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                            &[
                                "The value for \'",
                                "\' is invalid in the configuration: \'",
                                "\'",
                            ],
                            &match (&"no_short", &x) {
                                (arg0, arg1) => [
                                    ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Display::fmt),
                                    ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Display::fmt),
                                ],
                            },
                        ));
                        res
                    }))
                })
                .unwrap_or(::std::default::Default::default()),
            no_long: args
                .value_of("no_long")
                .or_else(|| file.get_from(Some("Defaults"), "no_long"))
                .and_then(|x| {
                    Some(x.parse::<bool>().expect(&{
                        let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                            &[
                                "The value for \'",
                                "\' is invalid in the configuration: \'",
                                "\'",
                            ],
                            &match (&"no_long", &x) {
                                (arg0, arg1) => [
                                    ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Display::fmt),
                                    ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Display::fmt),
                                ],
                            },
                        ));
                        res
                    }))
                })
                .unwrap_or(::std::default::Default::default()),
            no_short_no_long: args
                .value_of("no_short_no_long")
                .or_else(|| file.get_from(Some("Defaults"), "no_short_no_long"))
                .and_then(|x| {
                    Some(x.parse::<bool>().expect(&{
                        let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                            &[
                                "The value for \'",
                                "\' is invalid in the configuration: \'",
                                "\'",
                            ],
                            &match (&"no_short_no_long", &x) {
                                (arg0, arg1) => [
                                    ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Display::fmt),
                                    ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Display::fmt),
                                ],
                            },
                        ));
                        res
                    }))
                })
                .unwrap_or(::std::default::Default::default()),
            file: args
                .value_of("file")
                .or_else(|| file.get_from(Some("Defaults"), "file"))
                .and_then(|x| {
                    Some(x.parse::<bool>().expect(&{
                        let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                            &[
                                "The value for \'",
                                "\' is invalid in the configuration: \'",
                                "\'",
                            ],
                            &match (&"file", &x) {
                                (arg0, arg1) => [
                                    ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Display::fmt),
                                    ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Display::fmt),
                                ],
                            },
                        ));
                        res
                    }))
                })
                .unwrap_or(::std::default::Default::default()),
            long: args
                .value_of("long")
                .or_else(|| file.get_from(Some("Defaults"), "long"))
                .and_then(|x| {
                    Some(x.parse::<bool>().expect(&{
                        let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                            &[
                                "The value for \'",
                                "\' is invalid in the configuration: \'",
                                "\'",
                            ],
                            &match (&"long", &x) {
                                (arg0, arg1) => [
                                    ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Display::fmt),
                                    ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Display::fmt),
                                ],
                            },
                        ));
                        res
                    }))
                })
                .unwrap_or(::std::default::Default::default()),
            short: args
                .value_of("short")
                .or_else(|| file.get_from(Some("Defaults"), "short"))
                .and_then(|x| {
                    Some(x.parse::<bool>().expect(&{
                        let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                            &[
                                "The value for \'",
                                "\' is invalid in the configuration: \'",
                                "\'",
                            ],
                            &match (&"short", &x) {
                                (arg0, arg1) => [
                                    ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Display::fmt),
                                    ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Display::fmt),
                                ],
                            },
                        ));
                        res
                    }))
                })
                .unwrap_or(::std::default::Default::default()),
            combined: args
                .value_of("combined")
                .or_else(|| file.get_from(Some("Defaults"), "combined"))
                .and_then(|x| {
                    Some(x.parse::<bool>().expect(&{
                        let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                            &[
                                "The value for \'",
                                "\' is invalid in the configuration: \'",
                                "\'",
                            ],
                            &match (&"combined", &x) {
                                (arg0, arg1) => [
                                    ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Display::fmt),
                                    ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Display::fmt),
                                ],
                            },
                        ));
                        res
                    }))
                })
                .unwrap_or(::std::default::Default::default()),
        })
    }
    fn write_file(&self, path: &str) -> Result<(), ::structconf::Error> {
        let mut conf = ::ini::Ini::new();
        conf.with_section(Some("Defaults"))
            .set("no_short", self.no_short.to_string());
        conf.with_section(Some("Defaults"))
            .set("no_long", self.no_long.to_string());
        conf.with_section(Some("Defaults"))
            .set("no_short_no_long", self.no_short_no_long.to_string());
        conf.with_section(Some("Defaults"))
            .set("file", self.file.to_string());
        conf.with_section(Some("Defaults"))
            .set("long", self.long.to_string());
        conf.with_section(Some("Defaults"))
            .set("short", self.short.to_string());
        conf.with_section(Some("Defaults"))
            .set("combined", self.combined.to_string());
        conf.write_to_file(path)?;
        Ok(())
    }
}
#[allow(dead_code)]
fn main() {}
#[main]
pub fn main() -> () {
    extern crate test;
    test::test_main_static(&[])
}
