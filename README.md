<div align="center">

<h1>StructConf</h1>
<span><i>Combine argument parsing with a config file at compile time.</i></span>

<a href="https://github.com/vidify/structconf/actions"><img alt="Build Status" src="https://github.com/vidify/structconf/workflows/Continuous%20Integration/badge.svg"></a>
</div>

---

StructConf is a derive macro that allows you to combine argument parsing from [clap](https://github.com/clap-rs/clap) and config file parsing from [rust-ini](https://github.com/zonyitoo/rust-ini) at compile time. It's inspired by the argument parser [gumdrop](https://github.com/murarth/gumdrop), and developed to be used in [Vidify](https://github.com/vidify). Example:

```rust
#[derive(StructConf)]
struct Config {
    #[conf(help = "enable debug mode.")]
    pub debug: bool,
    #[conf(conf_file, no_short, help = "specify a config file.")]
    pub config_file: String,
}
```

Read [the docs]() for more details on how to use StructConf.
