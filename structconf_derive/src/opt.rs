use crate::error::Result;
use crate::attrs::BasicOptAttrs;

use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use std::rc::Rc;
use syn::{Expr, Field, Ident, Type};

// DATA STRUCTURES
pub struct OptBaseData {
    pub name: Ident,
    pub ty: Type,
    pub is_option: bool,
    pub default: Option<String>,
    pub takes_value: bool,
}

pub struct OptArgData {
    pub long: Option<String>,
    pub short: Option<String>,
    pub help: Option<String>,
    pub inverse: bool,
}

pub struct OptFileData {
    pub name: String,
    pub section: String,
}

pub trait OptData {
    fn parse(attrs: BasicOptAttrs) -> Result<Self>
        where
            Self: Sized;
}


// IMPLEMENTATION OF THE OPT TYPES
pub enum OptKind {
    Empty,
    Arg(OptArgData),
    File(OptFileData),
}

pub struct Opt {
    pub base: Rc<OptBaseData>,
    pub kind: OptKind
}

impl Opt {
    pub fn into_field_init(&self) -> Result<TokenStream2> {
        match &self.kind {
            OptKind::Empty => { },
            OptKind::Arg(arg) => { },
            OptKind::File(file) => { },
        }

        Ok(TokenStream2::new())
    }

    pub fn into_arg_init(&self) -> Option<TokenStream2> {
        match &self.kind {
            OptKind::Empty => { },
            OptKind::Arg(arg) => { },
            OptKind::File(file) => { },
        }

        Some(TokenStream2::new())
    }

    pub fn into_to_file(&self) -> Option<TokenStream2> {
        match &self.kind {
            OptKind::Empty => { },
            OptKind::Arg(arg) => { },
            OptKind::File(file) => { },
        }

        Some(TokenStream2::new())
    }
}
