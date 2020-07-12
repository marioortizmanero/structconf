//! Please visit [docs.rs/structconf](https://docs.rs/structconf/) for the
//! documentation.

extern crate darling;
extern crate proc_macro;

mod attrs;
mod error;
mod opt;

use crate::attrs::Attrs;
use crate::error::{Error, ErrorKind, Result};
use crate::opt::{Opt, OptKind};

use proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::quote;
use std::collections::HashSet;
use syn::{Data, DataStruct, DeriveInput, Fields, FieldsNamed};

#[proc_macro_derive(StructConf, attributes(conf))]
pub fn derive_conf(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(input).unwrap();
    let name = &ast.ident;
    let span = name.span();

    // Build the trait implementation
    let result = match ast.data {
        Data::Struct(DataStruct {
            fields: Fields::Named(named_fields),
            ..
        }) => impl_conf_macro(name, named_fields),
        Data::Struct(_) => Err(Error {
            kind: ErrorKind::DeriveType(String::from("unnamed struct")),
            span,
        }),
        Data::Enum(_) => Err(Error {
            kind: ErrorKind::DeriveType(String::from("enum")),
            span,
        }),
        Data::Union(_) => Err(Error {
            kind: ErrorKind::DeriveType(String::from("union")),
            span,
        }),
    };

    match result {
        Ok(tokens) => tokens,
        Err(e) => syn::Error::from(e).to_compile_error().into(),
    }
}

fn impl_conf_macro(name: &Ident, fields: FieldsNamed) -> Result<TokenStream> {
    // Obtaining the basic attributes from the fields.
    let mut options = Vec::new();
    let mut tok_fields = Vec::new();
    for field in fields.named.into_iter() {
        let attr = Attrs::init(field)?;
        let (opt1, opt2) = attr.parse_opt()?;

        // If both attributes were returned, `opt1` will be the arguments
        // and `opt2` will be the config file. Their name and default are
        // guaranteed to be the same.
        let name = &opt1.base.name;
        let default = &opt1.into_field_default()?;
        let first = &opt1.into_field_init()?;
        let second = if let Some(opt) = &opt2 {
            let tok = opt.into_field_init()?;
            // The statement should be `else if`.
            quote! { else #tok }
        } else {
            quote! {}
        };
        tok_fields.push(quote! {
            #name: {
                #first
                #second
                else { #default }
            }
        });

        options.push(opt1);
        if let Some(opt) = opt2 {
            options.push(opt);
        }
    }

    check_conflicts(&options)?;

    let mut tok_args = Vec::new();
    let mut tok_to_file = Vec::new();
    for opt in options {
        if let Some(tok) = opt.into_arg_init() {
            tok_args.push(tok);
        }
        if let Some(tok) = opt.into_to_file() {
            tok_to_file.push(tok);
        }
    }

    let trait_impl = quote! {
        impl StructConf for #name {
            fn parse(
                app: ::clap::App,
                path: &str
            ) -> Result<#name, ::structconf::Error>
                where
                    Self: Sized
            {
                let args = #name::parse_args(app);
                #name::parse_file(&args, path)
            }

            fn parse_args<'a>(
                app: ::clap::App<'a, 'a>
            ) -> ::clap::ArgMatches<'a> {
                #name::parse_args_from(
                    app,
                    &mut ::std::env::args()
                )
            }

            fn parse_args_from<'a, I, T>(
                app: clap::App<'a, 'a>,
                iter: I,
            ) -> clap::ArgMatches<'a>
                where
                    I: IntoIterator<Item = T>,
                    T: Into<::std::ffi::OsString> + Clone {
                app.args(&[
                    #(#tok_args,)*
                ]).get_matches_from(iter)
            }

            fn parse_file(
                args: &::clap::ArgMatches,
                path: &str
            ) -> Result<#name, ::structconf::Error> where Self: Sized {
                // Checking that the config file exists, and creating it
                // otherwise.
                let path_wrap = ::std::path::Path::new(path);
                if !path_wrap.exists() {
                    ::std::fs::File::create(&path_wrap)?;
                    eprintln!("Created config file at {}", path);
                }

                let file = ::ini::Ini::load_from_file(path)?;
                Ok(#name {
                    #(#tok_fields,)*
                })
            }

            fn write_file(
                &self,
                path: &str
            ) -> Result<(), ::structconf::Error> {
                let mut conf = ::ini::Ini::new();
                #(#tok_to_file)*
                conf.write_to_file(path)?;

                Ok(())
            }
        }
    };

    Ok(trait_impl.into())
}

// Looks for conflicts in the options as a whole, like repeated IDs.
fn check_conflicts(opts: &Vec<Opt>) -> Result<()> {
    let mut files = HashSet::new();
    let mut longs = HashSet::new();
    let mut shorts = HashSet::new();

    macro_rules! try_insert {
        ($iter:expr, $new:expr, $span:expr, $err_id:expr) => {
            if !$iter.insert($new) {
                return Err(Error {
                    kind: ErrorKind::ConflictIDs($err_id.to_string(), $new),
                    span: $span,
                });
            }
        };
    }

    for opt in opts {
        let span = opt.base.name.span();
        match &opt.kind {
            OptKind::Empty => {}
            OptKind::Flag(arg) | OptKind::Arg(arg) => {
                if let Some(short) = &arg.short {
                    try_insert!(shorts, short.clone(), span, "short");
                }
                if let Some(long) = &arg.long {
                    try_insert!(longs, long.clone(), span, "short");
                }
            }
            OptKind::File(file) => {
                try_insert!(files, file.name.clone(), span, "file");
            }
        }
    }

    Ok(())
}
