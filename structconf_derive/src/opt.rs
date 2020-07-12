use crate::error::Result;

use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use std::rc::Rc;
use syn::{Expr, Ident, Type};

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
    /// Expected output:
    ///
    /// TAKES VALUE:
    /// ```rust
    /// if let Some(val) = args.value_of(...) {
    ///     parse(val)
    /// }
    /// else if let Some(val) = file.get_from(...) {
    ///     parse(val)
    /// }
    /// else {
    ///     Ok(default)
    /// }
    /// ```
    ///
    /// DOESN'T TAKE VALUE:
    /// ```rust
    /// if args.is_present(...) {
    ///     // if `arg_inverse`, it's false
    ///     true
    /// }
    /// else if let Some(val) = file.get_from(...) {
    ///     val
    /// }
    /// else {
    ///     default
    /// }
    /// ```

    pub fn into_field_default(&self) -> Result<TokenStream2> {
        match &self.base.default {
            Some(expr) => {
                let expr = syn::parse_str::<Expr>(&expr)?;
                if self.base.is_option {
                    Ok(quote! { Some(#expr) })
                } else {
                    Ok(quote! { #expr })
                }
            }
            None => if self.base.is_option {
                Ok(quote! { None })
            } else {
                Ok(quote! { ::std::default::Default::default() })
            }
        }
    }

    pub fn into_field_init(&self) -> Result<TokenStream2> {
        let name = &self.base.name;
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
                let default = self.into_field_default()?;
                Ok(quote! {
                    if true {
                        #default
                    }
                })
            },
            OptKind::Arg(OptArgData { inverse, .. }) => {
                if self.base.takes_value {
                    Ok(quote! {
                        if let Some(val) = args.value_of(stringify!(#name)) {
                            #parse
                            #ret
                        }
                    })
                } else {
                    let ret = if *inverse {
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
            },
            OptKind::File(OptFileData { name, section }) => {
                Ok(quote! {
                    if let Some(val) = file.get_from(Some(#section), #name) {
                        #parse
                        #ret
                    }
                })
            },
        }
    }

    pub fn into_arg_init(&self) -> Option<TokenStream2> {
        let id = self.base.name.to_string();
        if let OptKind::Arg(OptArgData { help, long, short, .. }) = &self.kind {
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

            if self.base.takes_value {
                init.extend(quote! {
                    .takes_value(true)
                });
            }

            Some(init)
        } else {
            None
        }
    }

    pub fn into_to_file(&self) -> Option<TokenStream2> {
        let id = &self.base.name;
        if let OptKind::File(OptFileData { name, section }) = &self.kind {
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
        } else {
            None
        }
    }
}
