use crate::error::Result;
use crate::attrs::BasicOptAttrs;

use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
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
pub struct OptNone {
    pub base: OptBaseData,
}

pub struct OptArg {
    pub base: OptBaseData,
    pub arg: OptArgData,
}

pub struct OptFile {
    pub base: OptBaseData,
    pub file: OptFileData,
}

pub struct OptBoth {
    pub base: OptBaseData,
    pub arg: OptArgData,
    pub file: OptFileData,
}

pub trait Opt {
    fn base(&self) -> OptBaseData;
    fn into_field_init(&self) -> Result<TokenStream2>;
    fn into_arg_init(&self) -> Option<TokenStream2>;
    fn into_to_file(&self) -> Option<TokenStream2>;
}

impl Opt for OptNone {
    fn base(&self) -> OptBaseData { self.base }

    fn into_field_init(&self) -> Result<TokenStream2> {
        Ok(TokenStream2::new())
    }

    fn into_arg_init(&self) -> Option<TokenStream2> {
        None
    }

    fn into_to_file(&self) -> Option<TokenStream2> {
        None
    }
}

impl Opt for OptArg {
    fn base(&self) -> OptBaseData { self.base }

    fn into_field_init(&self) -> Result<TokenStream2> {
        Ok(TokenStream2::new())
    }

    fn into_arg_init(&self) -> Option<TokenStream2> {
        None
    }

    fn into_to_file(&self) -> Option<TokenStream2> {
        None
    }
}

impl Opt for OptFile {
    fn base(&self) -> OptBaseData { self.base }

    fn into_field_init(&self) -> Result<TokenStream2> {
        Ok(TokenStream2::new())
    }

    fn into_arg_init(&self) -> Option<TokenStream2> {
        None
    }

    fn into_to_file(&self) -> Option<TokenStream2> {
        None
    }
}

impl Opt for OptBoth {
    fn base(&self) -> OptBaseData { self.base }

    fn into_field_init(&self) -> Result<TokenStream2> {
        Ok(TokenStream2::new())
    }

    fn into_arg_init(&self) -> Option<TokenStream2> {
        None
    }

    fn into_to_file(&self) -> Option<TokenStream2> {
        None
    }
}
