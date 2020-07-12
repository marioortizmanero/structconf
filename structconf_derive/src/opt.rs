use crate::error::Result;
use crate::attrs::BasicOptAttrs;

use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{Expr, Field, Ident, Type};


/// The higher level structure with already processed data for an option.
pub struct Opt {
    pub name: Ident,
    pub ty: Type,
    pub is_option: bool,
    pub default: Option<String>,
    pub takes_value: bool,
    pub file: Option<OptFileData>,
    pub arg: Option<OptArgData>,
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

impl Opt {
    pub fn parse(field: Field) -> Result<Opt> {
        let attrs = BasicOptAttrs::init(field)?;

        Ok(Opt {
            file: attrs.parse_file(),
            arg: attrs.parse_arg()?,
            takes_value: attrs.takes_value(),
            is_option: attrs.is_option,
            default: attrs.default,
            name: attrs.ident.unwrap(),
            ty: attrs.ty,
        })
    }

    /// This will return a tokenstream for each field that has to be
    /// initialized, depending on whether the option is an argument, a
    /// config file option, none, or both:
    ///
    /// ```rust
    /// field: {
    ///     // May go through no branches, branch A, branch B, or both
    ///     // branches.
    ///     let mut opt: Option<String> = None;
    ///
    ///     // Branch A (argument)
    ///     opt = arg.value_of(...);
    ///
    ///     // Branch B (config file)
    ///     if let None = opt {
    ///         opt = file.get_from(...);
    ///     }
    ///
    ///     match opt {
    ///         // Branch A, B, or both
    ///         Some(opt) => {
    ///             // Parse `opt`
    ///             let opt = opt.parse()?;
    ///         
    ///             // Branch A, if `inverse_arg`, after parsing.
    ///             // `inverse_arg` // may only be true if it's an argument.
    ///             if arg.value_of(...).is_some() {
    ///                 opt = !opt;
    ///             }
    ///         },
    ///         // None
    ///         None => default
    ///     }
    /// }
    /// ```
    ///
    /// This is intended to be as flexible as possible, and to avoid
    /// repetition in the code, so it may not be optimized for some cases. For
    /// example, if the option is neither a config file option nor an
    /// argument, the `match` block is useless because it's obvious it's going
    /// to go through the `None` branch. But these cases are easy to optimize
    /// by the compiler, so it's not important.
    ///
    /// Methods like `unwrap_or` can't be used here because this has to be
    /// able to propagate errors with `?`.
    pub fn into_field_init(&self) -> Result<TokenStream2> {
        let Opt {
            name,
            ty,
            takes_value,
            is_option,
            arg,
            ..
        } = self;

        // Obtains a TokenStream with what the default value of the option
        // is going to be. If `default` was used, the expression is used.
        // Otherwise, the type's default value is used, which may be `None`
        // in case the type is an `Option<T>`.
        let default = match self.default.to_owned() {
            Some(expr) => {
                let expr = syn::parse_str::<Expr>(&expr)?;
                if *is_option {
                    quote! { Some(#expr) }
                } else {
                    quote! { #expr }
                }
            }
            None => {
                if *is_option {
                    quote! { None }
                } else {
                    quote! { ::std::default::Default::default() }
                }
            }
        };

        let val_ret = if *is_option {
            quote! { Some(opt) }
        } else {
            quote! { opt }
        };

        let mut value = if *takes_value {
            quote! {
                let mut opt: Option<&str> = None;
            }
        } else {
            quote! {
                let mut opt: bool = #default;
            }
        };

        if let Some(OptArgData { inverse, .. }) = arg {
            if *takes_value {
                value.extend(quote! {
                    opt = args.value_of(stringify!(#name));
                });
            } else {
                let flag_val = if !*inverse {
                    quote! { true }
                } else {
                    quote! { false }
                };
                value.extend(quote! {
                    if args.is_present(stringify!(#name)) {
                        opt = #flag_val
                    }
                });
            }
        }

        if let Some(OptFileData { name, section }) = &self.file {
            value.extend(quote! {
                if let None = opt {
                    opt = file.get_from(Some(#section), #name);
                }
            });
        }

        value.extend(quote! {
            match opt {
                Some(opt) => {
                    let mut opt = opt
                        .parse::<#ty>()
                        .map_err(|e| {
                            ::structconf::Error::Parse(e.to_string())
                        })?;
                    #val_ret
                },
                None => #default
            }
        });

        Ok(quote! {
            #name: { #value }
        })
    }

    pub fn into_arg_init(&self) -> Option<TokenStream2> {
        // In case it's not an argument, an empty TokenStream will be
        // returned.
        if let Some(data) = &self.arg {
            let name = self.name.to_string();
            let mut init = quote! {
                ::clap::Arg::with_name(#name)
            };

            if let Some(help) = &data.help {
                init.extend(quote! {
                    .help(#help)
                });
            }

            if let Some(long) = &data.long {
                init.extend(quote! {
                    .long(#long)
                });
            }

            if let Some(short) = &data.short {
                init.extend(quote! {
                    .short(#short)
                });
            }

            if self.takes_value {
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
        self.file.as_ref().and_then(|file| {
            let name = self.name.clone();
            let file_name = file.name.as_str();
            let section = file.section.as_str();

            Some(if self.is_option {
                quote! {
                    if let Some(val) = &self.#name {
                        conf.with_section(Some(#section))
                            .set(#file_name, val.to_string());
                    }
                }
            } else {
                quote! {
                    conf.with_section(Some(#section))
                        .set(#file_name, self.#name.to_string());
                }
            })
        })
    }
}
