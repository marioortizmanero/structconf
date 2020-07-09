<div align="center">

<h1>StructConf</h1>
<span><i>Combine argument parsing with a config file at compile time.</i></span>

<a href="https://github.com/vidify/structconf/actions"><img alt="Build Status" src="https://github.com/vidify/structconf/workflows/Continuous%20Integration/badge.svg"></a>
</div>

---

StructConf is a derive macro that allows you to combine argument parsing from [clap](https://github.com/clap-rs/clap) and config file parsing from [rust-ini](https://github.com/zonyitoo/rust-ini) at compile time. It's inspired by the argument parser [gumdrop](https://github.com/murarth/gumdrop), and developed to be used in [Vidify](https://github.com/vidify). Example:

```rust
use structconf::StructConf;

#[derive(StructConf)]
struct Config {
    // Option available in the config file and the arguments
    #[conf(help = "description for the argument parser.")]
    pub default: i32,
    // Specify where the options are available.
    #[conf(no_file)]
    pub args_opt: u8,
    #[conf(no_short, no_long)]
    pub conf_opt: Option<String>,
    #[conf(no_short, no_long, no_file)]
    pub ignored: bool,
    // Customize the names
    #[conf(short = "x", long = "renamed-opt", file = "my_opt",
           help = "custom names.")]
    pub renamed: String,
    // Inverse arguments
    #[conf(short = "n", long = "no_pancakes", help = "disable pancakes.")]
    pub pancakes: bool,
    // Custom default values
    #[conf(default = "123.45")]
    pub floating: f64,
}
```

Read [the docs]() for more details on how to use StructConf.
