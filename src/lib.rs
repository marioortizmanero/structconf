use std::sync::RwLock;
pub use structconf_derive::StructConf;

pub trait StructConf {
    fn new() -> RwLock<Self>;
    fn to_file(&self);
}
