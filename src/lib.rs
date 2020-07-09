//! StructConf is a derive macro to combine argument parsing from
//! [clap](https://github.com/clap-rs/clap) and config file parsing from
//! [rust-ini](https://github.com/zonyitoo/rust-ini) at compile time. For
//! example:
//!
//! ```rust
//! use structconf::StructConf;
//!
//! #[derive(StructConf)]
//! struct Config {
//!     // Option available in the config file and the arguments
//!     #[conf(help = "description for the argument parser.")]
//!     pub default: i32,
//!     // Specify where the options are available.
//!     #[conf(no_file)]
//!     pub args_opt: u8,
//!     #[conf(no_short, no_long)]
//!     pub conf_opt: Option<String>,
//!     #[conf(no_short, no_long, no_file)]
//!     pub ignored: bool,
//!     // Customize the names
//!     #[conf(short = "x", long = "renamed-opt", file = "my_opt",
//!            help = "custom names.")]
//!     pub renamed: String,
//!     // Inverse arguments
//!     #[conf(short = "n", long = "no_pancakes", help = "disable pancakes.")]
//!     pub pancakes: bool,
//!     // Custom default values
//!     #[conf(default = "123.45")]
//!     pub floating: f64,
//! }
//! ```
//!
//! Additional attributes can be added to its fields to customize how they are
//! parsed:
//!
//! ## General attributes
//! * `default = "..."`: a Rust expression that will be evaluated when the
//! option isn't found in the argument parser or the config file. For example,
//! `default = "1+2"`, or `default = "hello"`. Otherwise, the value given by
//! [`std::default::Default`](https://doc.rust-lang.org/std/default/trait.Default.html)
//! will be used.
//!
//! ## Argument parser attributes
//! * `help = "..."`: the help message shown in the argument parser.
//! * `long = "arg_name"`: a custom long argument name. Otherwise, it will be
//! obtained directly from the field's name. `do_something` will be
//! `--do-something`.
//! * `no_long`: the option won't include the option as a long argument.
//! * `short = "x"`: a custom short argument name (only made up of a single
//! character). Otherwise, it will be obtained directly from the field's
//! name. `do_something` will be `-d`.
//! * `no_short`: the option won't include the option as a short argument.
//! * `inverse_arg`: the argument value is the opposite. This is useful for
//! arguments where the argument's value is inverted:
//!
//! ```rust
//! use structconf::StructConf;
//!
//! #[derive(StructConf)]
//! struct Bakery {
//!     #[conf(inverse_arg, no_short, long = "--no-pancakes")]
//!     pancakes: bool
//! }
//! ```
//!
//! If both `no_long` and `no_short` are provided, the option won't be
//! available in the argument parser.
//!
//! ## Config file attributes
//! * `file = "..."`: a custom option name for the config file. Otherwise, it
//! will be the same as the field's name.
//! * `no_file` can be provided to not have the option available in the config
//! file.
//! * `section`: the section in the config file where the option will be.
//! Otherwise, `Default` is used. For example,
//! `#[structconf(section = "Planes")] model_id: i32` will look like this in
//! the config file:
//!
//! ```ini
//! [Planes]
//! model_id = 123
//! ```

pub use structconf_derive::StructConf;

use std::fmt;
use std::io;

/// Small wrapper for the possible errors that may occur when parsing a
/// StructConf-derived struct.
#[derive(Debug)]
pub enum Error {
    IO(io::Error),
    Ini(ini::ini::ParseError),
    Parse(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            Error::IO(err) => err.fmt(f),
            Error::Ini(err) => err.fmt(f),
            Error::Parse(msg) => write!(f, "Error when parsing: {}", msg),
        }
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Error::IO(err)
    }
}

impl From<ini::ini::Error> for Error {
    fn from(err: ini::ini::Error) -> Self {
        match err {
            ini::ini::Error::Io(err) => Error::IO(err),
            ini::ini::Error::Parse(err) => Error::Ini(err),
        }
    }
}

pub trait StructConf {
    /// Instantiate the structure from both the argument parser and the
    /// config file. This can also be done in two steps with `parse_args`
    /// and `parse_config`, which makes it possible to have an argument
    /// with the config file path.
    fn parse(app: clap::App, path: &str) -> Result<Self, Error>
    where
        Self: Sized;
    /// Parses only the arguments with clap.
    fn parse_args<'a>(app: clap::App<'a, 'a>) -> clap::ArgMatches<'a>;
    /// With the argument matches returned by `parse_args`, the config
    /// file is read, and the struct is constructed with the default values
    /// taken into account.
    ///
    /// This also serves as a function to refresh the config file values.
    fn parse_file(args: clap::ArgMatches, path: &str) -> Result<Self, Error>
    where
        Self: Sized;
    /// Writes *all* the config file options into a file.
    fn write_file(&self, path: &str) -> Result<(), Error>;
}
