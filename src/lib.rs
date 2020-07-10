//! StructConf is a derive macro to combine argument parsing from
//! [clap](https://docs.rs/clap/) and config file parsing from [rust-ini](
//! https://docs.rs/rust-ini/) at compile time. For example:
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
//! Any named struct that uses `#[derive(StructConf)]` will have the methods
//! from [`structconf::StructConf`](
//! https://docs.rs/structconf/latest/structconf/trait.StructConf.html)
//! available.
//!
//! Additional attributes can be added to its fields to customize how they
//! are parsed:
//!
//! ## General attributes
//! * `default = "..."`: a Rust expression that will be evaluated as a
//! fallback value. For example, `default = "1+2"`, or
//! `default = "String::from(\"hello\"")`. Otherwise, the value given by
//! [`std::default::Default`](
//! https://doc.rust-lang.org/std/default/trait.Default.html) will be used,
//! or in case the assigned type is `Option<T>`\*, `None`.
//!
//! \* *Note: the assigned type must be exactly `Option<T>` for this to work.
//! `std::option::Option<T>` won't work, for example.*
//!
//! ## Argument parser attributes
//! * `help = "..."`: the help message shown in the argument parser when
//! `--help` is used.
//! * `long = "arg_name"`: a custom long argument name. Otherwise, it will be
//! obtained directly from the field's name. `do_something` will be
//! `--do-something`.
//! * `no_long`: don't include the option as a long argument.
//! * `short = "x"`: a custom short argument name (only made up of a single
//! character). Otherwise, it will be obtained directly from the field's
//! name. `do_something` will be `-d`.
//! * `no_short`: don't include the option as a short argument.
//! * `inverse_arg`: the argument value is the opposite. The assigned type
//! must implement [`std::ops::Not`](
//! https://doc.rust-lang.org/std/ops/trait.Not.html). This is useful for
//! arguments where the argument's value is negated:
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
//! available in the argument parser at all.
//!
//! ## Config file attributes
//! * `file = "..."`: a custom option name for the config file. Otherwise, it
//! will be the same as the field's name.
//! * `no_file`: son't include the option in the config file.
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

/// This trait implements the methods available after using
/// `#[derive(StructConf)]`:
pub trait StructConf {
    /// Instantiate the structure from both the argument parser and the
    /// config file, falling back to the default values.
    ///
    /// The priority followed for the configuration is "arguments >
    /// config file > default values".
    ///
    /// The `path` argument is where the config file will be. If it doesn't
    /// exist, it will be created, and a message to stderr will be printed.
    fn parse(app: clap::App, path: &str) -> Result<Self, Error>
    where
        Self: Sized;

    /// Parses only the arguments with [clap](
    /// https://docs.rs/clap/2.33.1/clap/). This is useful for a
    /// `--config-file` argument to allow the user to choose the config
    /// file location.
    fn parse_args<'a>(app: clap::App<'a, 'a>) -> clap::ArgMatches<'a>;

    /// With the argument matches returned by `parse_args`, the config
    /// file is read, and the struct is initialized with the default values
    /// taken into account.
    ///
    /// This also serves as a function to refresh the config file values.
    fn parse_file(args: &clap::ArgMatches, path: &str) -> Result<Self, Error>
    where
        Self: Sized;

    /// Writes the structure values into a config file.
    fn write_file(&self, path: &str) -> Result<(), Error>;
}
