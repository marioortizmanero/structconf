//! StructConf is a derive macro to combine argument parsing from
//! [clap](https://docs.rs/clap/) and config file parsing from [rust-ini](
//! https://docs.rs/rust-ini/) at compile time. Here's a very simple example (see
//! more detailed examples on [the GitHub repo](
//! https://github.com/vidify/structconf/tree/master/examples)):
//!
//! ```rust
//! use clap::App;
//! use structconf::StructConf;
//!
//! #[derive(Debug, StructConf)]
//! struct ServerConfig {
//!     #[conf(help = "The public key")]
//!     pub public_key: String,
//!     #[conf(no_file, long = "your-secret", help = "Your secret API key")]
//!     pub secret_key: String,
//!     #[conf(default = "100", help = "timeout in seconds")]
//!     pub timeout: i32,
//! }
//!
//! let app = App::new("demo");
//! let conf = ServerConfig::parse(app, "config.ini");
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
//! * `negated_arg`: the flag's value is the opposite:
//!
//! ```rust
//! use structconf::StructConf;
//!
//! #[derive(StructConf)]
//! struct Bakery {
//!     // By default it's `true`, unless `--no-pancakes` is passed.
//!     #[conf(negated_arg, no_short, long = "no-pancakes")]
//!     pancakes: bool
//! }
//! ```
//!
//! If both `no_long` and `no_short` are provided, the option won't be
//! available in the argument parser at all.
//!
//! ## Config file attributes
//! * `file = "..."`: set a custom name in the config file. Otherwise, it will
//! be the same as the field's identifier.
//! * `no_file`: don't include the option in the config file.
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

use std::ffi::OsString;
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
/// `#[derive(StructConf)]`.
///
/// The priority followed for the configuration is "arguments > config file >
/// default values".
pub trait StructConf {
    /// Instantiate the structure from both the argument parser and the
    /// config file, falling back to the default values. Equivalent to
    /// calling `parse_args` and then `parse_file`.
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
    ///
    /// This is equivalent to `parse_args_from(..., &mut std::env::args())`.
    fn parse_args<'a>(app: clap::App<'a, 'a>) -> clap::ArgMatches<'a>;

    /// Parses only the arguments with [clap](
    /// https://docs.rs/clap/2.33.1/clap/) from an iterator.
    fn parse_args_from<'a, I, T>(app: clap::App<'a, 'a>, iter: I) -> clap::ArgMatches<'a>
    where
        I: IntoIterator<Item = T>,
        T: Into<OsString> + Clone;

    /// The config file is read after parsing the arguments, and the struct
    /// is initialized with the default values taken into account.
    ///
    /// The `path` argument is where the config file will be. If it doesn't
    /// exist, it will be created, and a message to stderr will be printed.
    ///
    /// This also serves as a function to refresh the config file values.
    fn parse_file(args: &clap::ArgMatches, path: &str) -> Result<Self, Error>
    where
        Self: Sized;

    /// Writes the structure's values into a config file, except for those
    /// that are wrapped by `Option` and whose value is `None`.
    fn write_file(&self, path: &str) -> Result<(), Error>;
}
