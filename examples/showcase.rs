//! This example shows how each of the available attributes is used.
//! You can run `cargo run --example showcase --` followed by any
//! flags you'd like to pass, or create `./config.ini` to test out the
//! config file.

use clap::App;
use structconf::StructConf;
use strum_macros::{Display, EnumString};

#[derive(Debug, Display, EnumString)]
pub enum Count {
    One,
    Two,
    Three,
}

impl Default for Count {
    fn default() -> Self {
        Count::One
    }
}

#[derive(Debug, StructConf)]
struct Config {
    // Option available in the config file and the argument parser
    #[conf(help = "description for the argument parser.")]
    pub default: i32,
    // Option only for the argument parser
    #[conf(no_file, help = "not available in the config file")]
    pub args_opt: u8,
    // Option only for the config file
    #[conf(no_short, no_long)]
    pub conf_opt: String,
    // Skipped fields
    #[conf(no_short, no_long, no_file)]
    pub ignored: bool,
    // Configurable names
    #[conf(
        short = "x",
        long = "renamed-opt",
        file = "my_opt",
        help = "custom names."
    )]
    pub renamed: String,
    // Negated arguments
    #[conf(
        negated_arg,
        short = "n",
        long = "no-pancakes",
        help = "disable pancakes."
    )]
    pub pancakes: bool,
    // Custom default values
    #[conf(help = "default: 123.45", default = "123.45")]
    pub floating: f64,
    // Optional values
    #[conf(no_short, no_long)]
    pub optional_field: Option<String>,
    #[conf(help = "custom type")]
    // Custom types
    pub count: Count,
}

pub fn main() {
    let app = App::new("demo");
    let conf = Config::parse(app, "config.ini");
    println!("Parsed config: {:#?}", conf);
}
