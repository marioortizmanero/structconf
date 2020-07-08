//! StructConf allows you to combine argument parsing from
//! [clap](https://github.com/clap-rs/clap) and config file parsing from
//! [rust-ini](https://github.com/zonyitoo/rust-ini) at compile time. For
//! example:
//!
//! ```rust
//! #[structonf]
//! struct Config {
//!     #[structconf(help = "enable debug mode.")]
//!     pub debug: bool,
//!     #[structconf(conf_file, no_short, help = "specify a config file.")]
//!     pub config_file: String,
//! }
//! ```
//!
//! This will implement the following public functions:
//!
//! ```rust
//! fn new<'a>(app: clap::App<'a, 'a>, default_conf_path: &str) -> Config;
//! ```
//!
//! The `#[structconf]` attribute is required on a named `struct`. It must
//! also be accompanied by the parameter `conf_path`, with the relative path
//! where the config file goes by default, separated by `/`. This path will
//! be appended to the user's config directory, given by the
//! [dirs-rs](https://docs.rs/dirs/3.0.1/dirs/fn.config_dir.html) crate. For
//! example, `conf_path = "myapp/conf.ini"` will result in
//! `/home/alice/.config/myapp/conf.ini` on Linux, and
//! `C:\Users\Alice\AppData\Roaming\myapp\conf.ini` on Windows.
//!
//! Additional attributes can be added to its fields to customize how they are
//! parsed:
//!
//! ## General attributes
//! * `conf_file`: special case to load the config file from a different path.
//! It will only be available as an argument, and will override the default
//! path given with `conf_path`.
//! * `default = "..."`: a Rust expression that will be evaluated when the
//! option isn't found in the argument parser or the config file. For example,
//! `default = "1+2"`, or `default = "hello"`. Otherwise, the value given by
//! [`std::default::Default`](https://doc.rust-lang.org/std/default/trait.Default.html)
//! will be used.
//!
//! ## Argument parser attributes
//! * `help = "..."`: the help message shown in the argument parser.
//! * `long = "..."`: a custom long argument name. Otherwise, it will be
//! obtained directly from the field's name (`do_something` will be
//! `--do-something`).
//! * `no_long`: the option won't include the option as a long argument.
//! * `short = "..."`: a custom short argument name. Otherwise, it will be
//! obtained directly from the field's name (`do_something` will be `-d`).
//! * `no_short`: the option won't include the option as a short argument.
//! * `arg_inverted`: the argument value is the opposite. This is useful for
//! arguments where the argument's value is inverted:
//!
//! ```rust
//! #[structconf(arg_inverted, no_short, long = "--no-pancakes")]
//! pancakes: bool
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

pub trait StructConf {
    /// Instantiate the structure from both the argument parser and the
    /// config file. This can also be done in two steps with `parse_args`
    /// and `parse_config`, which makes it possible to have an argument
    /// with the config file path.
    fn parse(app: clap::App, path: &str) -> Self;
    /// Parses only the arguments with clap.
    fn parse_args<'a>(app: clap::App<'a, 'a>) -> clap::ArgMatches<'a>;
    /// With the argument matches returned by `parse_args`, the config
    /// file is read, and the struct is constructed with the default values
    /// taken into account.
    ///
    /// This also serves as a function to refresh the config file values.
    fn parse_file(args: clap::ArgMatches, path: &str) -> Self;
    /// Writes *all* the config file options into a file.
    fn write_file(&self, path: &str);
}
