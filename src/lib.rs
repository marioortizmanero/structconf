pub use structconf_derive::StructConf;

pub trait StructConf {
    fn new(app: clap::App) -> Self;
    fn to_file(&self);
    fn refresh_file(&mut self);
}
