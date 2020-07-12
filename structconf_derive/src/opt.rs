//! A higher level structure to store data about an option, intended to
//! be converted from an `Attrs`, and then parsed into token streams with
//! actual generated code.
//!
//! The options are stored individually, meaning that a field in the derived
//! struct can represent multiple options, like fields that are both a config
//! file option and an argument.

use crate::error::Result;

use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use std::rc::Rc;
use syn::{Expr, Ident, Type};

pub struct OptBaseData {
    pub id: Ident,
    pub ty: Type,
    pub is_option: bool,
    pub default: Option<String>,
}

pub struct OptArgData {
    pub long: Option<String>,
    pub short: Option<String>,
    pub help: Option<String>,
    pub negated: bool,
}

pub struct OptFileData {
    pub name: String,
    pub section: String,
}

pub enum OptKind {
    /// Not an option
    Empty,
    /// An argument that takes value
    Arg(OptArgData),
    /// An argument that doesn't take value
    Flag(OptArgData),
    /// A config file option
    File(OptFileData),
}

pub struct Opt {
    /// As an option can share multiple fields, the base data is shared
    /// among multiple of them.
    pub base: Rc<OptBaseData>,
    /// This field contains information specific to the type of argument
    /// it is.
    pub kind: OptKind,
}

impl Opt {
    /// Generates the default value the option will take as a fallback.
    pub fn gen_default(&self) -> Result<TokenStream2> {
        match &self.base.default {
            Some(expr) => {
                let expr = syn::parse_str::<Expr>(&expr)?;

                if self.base.is_option {
                    Ok(quote! { Some(#expr) })
                } else {
                    Ok(quote! { #expr })
                }
            }
            None => {
                // Negated flags are always true by default. They also can't
                // have a `default` field.
                if let OptKind::Flag(OptArgData { negated: true, .. }) =
                    self.kind
                {
                    Ok(quote! { true })
                } else if self.base.is_option {
                    Ok(quote! { None })
                } else {
                    Ok(quote! { ::std::default::Default::default() })
                }
            }
        }
    }

    /// Generates the field initialization logic. This may read data from the
    /// config file or the argument parser results following the structure
    /// found in the main file's `impl_conf_macro`, which combines all the
    /// options for a field in order.
    pub fn gen_field_init(&self) -> Result<TokenStream2> {
        let name = &self.base.id;
        let ty = &self.base.ty;
        let parse = quote! {
            let val = val
                .parse::<#ty>()
                .map_err(|e| {
                    ::structconf::Error::Parse(e.to_string())
                })?;
        };
        let ret = if self.base.is_option {
            quote! { Some(val) }
        } else {
            quote! { val }
        };

        match &self.kind {
            OptKind::Empty => {
                let default = self.gen_default()?;
                Ok(quote! {
                    if true {
                        #default
                    }
                })
            }
            OptKind::Flag(OptArgData { negated, .. }) => {
                let ret = if *negated {
                    quote! { false }
                } else {
                    quote! { true }
                };

                Ok(quote! {
                    if args.is_present(stringify!(#name)) {
                        #ret
                    }
                })
            }
            OptKind::Arg(_) => Ok(quote! {
                if let Some(val) = args.value_of(stringify!(#name)) {
                    #parse
                    #ret
                }
            }),
            OptKind::File(OptFileData { name, section }) => Ok(quote! {
                if let Some(val) = file.get_from(Some(#section), #name) {
                    #parse
                    #ret
                }
            }),
        }
    }

    /// Generates the argument initialization logic for `clap`. This will
    /// only work for options that represent an argument.
    pub fn gen_arg_init(&self) -> Option<TokenStream2> {
        match &self.kind {
            OptKind::Arg(OptArgData {
                help, long, short, ..
            })
            | OptKind::Flag(OptArgData {
                help, long, short, ..
            }) => {
                let id = self.base.id.to_string();
                let mut init = quote! {
                    ::clap::Arg::with_name(#id)
                };

                if let Some(help) = help {
                    init.extend(quote! {
                        .help(#help)
                    });
                }

                if let Some(long) = long {
                    init.extend(quote! {
                        .long(#long)
                    });
                }

                if let Some(short) = short {
                    init.extend(quote! {
                        .short(#short)
                    });
                }

                if let OptKind::Arg(_) = self.kind {
                    init.extend(quote! {
                        .takes_value(true)
                    });
                }

                Some(init)
            }
            _ => None,
        }
    }

    /// Generates the logic to write to a config file with `rust-ini`. This
    /// will only work for options available in the config file.
    pub fn gen_write_file(&self) -> Option<TokenStream2> {
        match &self.kind {
            OptKind::File(OptFileData { name, section }) => {
                let id = &self.base.id;
                if self.base.is_option {
                    Some(quote! {
                        if let Some(val) = &self.#id {
                            conf.with_section(Some(#section))
                                .set(#name, val.to_string());
                        }
                    })
                } else {
                    Some(quote! {
                        conf.with_section(Some(#section))
                            .set(#name, self.#id.to_string());
                    })
                }
            }
            _ => None,
        }
    }
}
