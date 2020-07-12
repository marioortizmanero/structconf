//! Please visit [docs.rs/structconf](https://docs.rs/structconf/) for the
//! documentation.

extern crate darling;
extern crate proc_macro;

mod error;
mod opt;

use crate::error::{Error, ErrorKind, Result};
use crate::opt::Opt;

use proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::quote;
use std::collections::HashSet;
use syn::{Data, DataStruct, DeriveInput, Fields, FieldsNamed};

#[proc_macro_derive(StructConf, attributes(conf))]
pub fn derive_conf(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast: DeriveInput = syn::parse(input).unwrap();
    let name = &ast.ident;

    // Build the trait implementation
    let result: Result<TokenStream> = match ast.data {
        Data::Struct(DataStruct {
            fields: Fields::Named(named_fields),
            ..
        }) => impl_conf_macro(name, named_fields),
        Data::Struct(_) => Err(Error {
            kind: ErrorKind::DeriveType(String::from("unnamed struct")),
            span: ast.ident.span(),
        }),
        Data::Enum(_) => Err(Error {
            kind: ErrorKind::DeriveType(String::from("enum")),
            span: ast.ident.span(),
        }),
        Data::Union(_) => Err(Error {
            kind: ErrorKind::DeriveType(String::from("union")),
            span: ast.ident.span(),
        }),
    };

    match result {
        Ok(tokens) => tokens.into(),
        Err(e) => syn::Error::from(e).to_compile_error().into(),
    }
}

fn impl_conf_macro(name: &Ident, fields: FieldsNamed) -> Result<TokenStream> {
    // The fields are parsed into a higher level structure to translate them
    // into the corresponding `TokenStream`s.
    let mut options = Vec::new();
    for field in fields.named.into_iter() {
        options.push(Opt::parse(field)?);
    }
    check_conflicts(&options)?;

    // The iterables needed for the StructConf methods are translated to use
    // them in the final `quote!`.
    let mut tok_fields = Vec::new();
    let mut tok_args = Vec::new();
    let mut tok_to_file = Vec::new();
    for opt in options {
        tok_fields.push(opt.into_field_init()?);
        if let Some(tok) = opt.into_arg_init() {
            tok_args.push(tok);
        }
        if let Some(tok) = opt.into_to_file() {
            tok_to_file.push(tok);
        }
    }

    Ok(quote! {
        impl StructConf for #name {
            fn parse(
                app: ::clap::App,
                path: &str
            ) -> Result<#name, ::structconf::Error> where Self: Sized {
                let args = #name::parse_args(app);
                #name::parse_file(&args, path)
            }

            // Using `parse_args_from` reduces the size of the binary
            // considerably.
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
    }
    .into())
}

// Looks for conflicts in the options as a whole, like repeated IDs.
fn check_conflicts(opts: &Vec<Opt>) -> Result<()> {
    let mut names = HashSet::new();
    let mut longs = HashSet::new();
    let mut shorts = HashSet::new();

    macro_rules! try_insert {
        ($iter:expr, $new:expr, $span:expr, $err_id:expr) => {
            if !$iter.insert($new) {
                return Err(Error {
                    kind: ErrorKind::ConflictIDs($err_id.to_string(), $new),
                    span: $span
                })
            }
        }
    }

    for opt in opts {
        let span = opt.name.span();
        try_insert!(names, opt.name.to_string(), span, "file");

        if let Some(arg) = opt.arg.as_ref() {
            if let Some(val) = arg.long.as_ref() {
                try_insert!(longs, val.clone(), span, "long");
            }
            if let Some(val) = arg.short.as_ref() {
                try_insert!(shorts, val.clone(), span, "short");
            }
        }
    }

    Ok(())
}
