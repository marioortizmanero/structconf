use structconf::StructConf;

#[derive(Debug, PartialEq, StructConf)]
struct Config {
    #[conf(no_file)]
    no_file: i32,
    #[conf(no_short)]
    no_short: bool,
    #[conf(no_long, short = "x")]
    no_long: bool,
    #[conf(no_short, no_long)]
    no_short_no_long: bool,
    #[conf(file = "new_file")]
    file: bool,
    #[conf(long = "name")]
    long: bool,
    #[conf(short = "s")]
    short: bool,
    #[conf(file = "new_combined", long = "new_combined", short = "c")]
    combined: bool,
}

fn main() {}
