<div align="center">

<h1>StructConf</h1>
<span><i>Combine argument parsing with a config file at compile time.</i></span>

<a href="https://github.com/vidify/structconf/actions"><img alt="Build Status" src="https://github.com/vidify/structconf/workflows/Continuous%20Integration/badge.svg"></a>
</div>

---

StructConf allows you to combine argument parsing from [clap](https://github.com/clap-rs/clap) and config file parsing from [rust-ini](https://github.com/zonyitoo/rust-ini) at compile time. Example:

```rust
#[structonf(conf_path = "myapp/conf.ini")]
struct Config {
    #[structconf(help = "enable debug mode.")]
    pub debug: bool,
    #[structconf(conf_file, no_short, help = "specify a config file.")]
    pub config_file: String,
}
```

The `#[structconf]` attribute is required on a named `struct`. It must also be accompanied by the parameter `conf_path`, with the relative path where the config file goes by default, separated by `/`. This path will be appended to the user's config directory given by the [dirs-rs](https://docs.rs/dirs/3.0.1/dirs/fn.config_dir.html) crate. For example, `conf_path = "myapp/conf.ini"` will result in `/home/alice/.config/myapp/conf.ini` on Linux, and `C:\Users\Alice\AppData\Roaming` on Windows.

Additional attributes can be added to its fields to customize how they are parsed:

## General
* `conf_file`: special case to load the config file from a different path. It will only be available as an argument, and will override the default path given with `conf_path`.
* `default = "..."`: a Rust expression that will be evaluated when the option isn't found in the argument parser or the config file. For example, `default = "1+2"`, or `default = "hello"`. Otherwise, the value given by [`std::default::Default`](https://doc.rust-lang.org/std/default/trait.Default.html) will be used.

## Argument parser
* `help = "..."`: the help message shown in the argument parser.
* `long = "..."`: a custom long argument name. Otherwise, it will be obtained directly from the field's name (`do_something` -> `--do-something`). If `no_long` is provided, there will be no long argument.
* `short = "..."`: a custom short argument name. Otherwise, it will be obtained directly from the field's name (`do_something` -> `-d`). `no_short` can be provided to not have a short argument.

If both `no_long` and `no_short` are provided, the option won't be available in the argument parser.

## Config file
* `file = "..."`: a custom option name for the config file. Otherwise, it will be the same as the field's name. `no_file` can be provided to not have it available in the config file.
* `section`: the section in the config file where the option will be. Otherwise, `Default` is used. For example, `#[structconf(section = "Planes")] model_id: i32` will look like this in the config file:

```ini
[Planes]
model_id = 123
```
