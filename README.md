<div align="center">

<h1>StructConf</h1>
<span><i>Combine argument parsing with a config file at compile time.</i></span>

<a href="https://github.com/vidify/structconf/actions"><img alt="Build Status" src="https://github.com/vidify/structconf/workflows/Continuous%20Integration/badge.svg"></a> <a alt="crates.io version" href="https://crates.io/crates/structconf"><img src="https://img.shields.io/crates/v/structconf.svg"></a> <a href="https://docs.rs/structconf"><img alt="docs.rs version" src="https://docs.rs/structconf/badge.svg"></a>
</div>

---

StructConf is a small derive macro that allows you to combine argument parsing from [clap](https://github.com/clap-rs/clap) and config file parsing from [rust-ini](https://github.com/zonyitoo/rust-ini) at compile time. It's inspired by the argument parser [structopt](https://github.com/TeXitoi/structopt), and developed to be used in [Vidify](https://github.com/vidify).

StructConf aims to be relatively small and simple. Here are its current selling points:

* Options available in the config file, argument parser, both, or none.
* Configurable option names.
* Custom types supported.
* Optional fields with `Option`.
* Custom default expressions.
* Insightful error messages.
* Thoroughly tested.

Small example:

```rust
use clap::App;
use structconf::StructConf;

#[derive(Debug, StructConf)]
struct ServerConfig {
    #[conf(help = "The public key")]
    pub public_key: String,
    #[conf(no_file, long = "your-secret", help = "Your secret API key")]
    pub secret_key: String,
    #[conf(default = "100", help = "timeout in seconds")]
    pub timeout: i32,
}

pub fn main() {
    let app = App::new("demo");
    let conf = ServerConfig::parse(app, "config.ini");
    println!("Parsed config: {:#?}", conf);
}
```

For more detauils on how to use Structconf, read [the docs](https://docs.rs/structconf/) and check out the [examples](examples).
