pub use structconf_derive::StructConf;
use std::sync::RwLock;

pub trait StructConf {
    fn new() -> RwLock<Self>;
    fn to_file(&self);
}

