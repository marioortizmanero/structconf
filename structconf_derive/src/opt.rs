use crate::error::{Error, ErrorKind, Result};

use darling::FromField;
use quote::quote;
use syn::{spanned::Spanned, Expr, Field, Ident, Type};

pub struct OptArgData {
    // An argument option may contain a long name, a short name, or both.
    pub long: Option<String>,
    pub short: Option<String>,
    pub help: Option<String>,
    pub inverse: bool,
}

pub struct OptFileData {
    pub section: String,
}

pub struct Opt {
    pub name: Ident,
    pub ty: Type,
    pub default: proc_macro2::TokenStream,
    pub file: Option<OptFileData>,
    pub arg: Option<OptArgData>,
}

#[derive(Debug, FromField)]
#[darling(attributes(conf))]
struct BasicOptAttrs {
    ident: Option<Ident>,
    ty: Type,
    #[darling(default)]
    default: Option<String>,
    #[darling(default)]
    no_long: bool,
    #[darling(default)]
    long: Option<String>,
    #[darling(default)]
    no_short: bool,
    #[darling(default)]
    short: Option<String>,
    #[darling(default)]
    help: Option<String>,
    #[darling(default)]
    inverse_arg: bool,
    #[darling(default)]
    no_file: bool,
    #[darling(default)]
    file: Option<String>,
    #[darling(default)]
    section: Option<String>,
}

impl BasicOptAttrs {
    fn check_conflicts(&self) -> Result<()> {
        // Given an original expression and a list of other expressions it
        // conflicts with, it returns an error in case both of them are
        // true. The macro makes this a bit less repetitive.
        macro_rules! check_conflicts {
            ($orig:expr, $others:expr) => {
                let (orig, orig_name) = $orig;
                if orig {
                    for (confl, confl_name) in $others.iter() {
                        if *confl {
                            return Err(Error {
                                span: self.ident.span(),
                                kind: ErrorKind::Conflict(
                                    orig_name.to_string(),
                                    confl_name.to_string()
                                )
                            });
                        }
                    }
                }
            };
        }

        // Empty fields may not use any other attribute
        check_conflicts!(
            (
                self.no_short && self.no_long && self.no_file,
                "no_short, no_long and no_file"
            ),
            [
                (self.default.is_some(), "default"),
                (self.long.is_some(), "long"),
                (self.short.is_some(), "short"),
                (self.help.is_some(), "help"),
                (self.inverse_arg, "inverse_arg"),
                (self.file.is_some(), "file"),
                (self.section.is_some(), "section"),
            ]
        );

        check_conflicts!(
            (self.no_short, "no_short"),
            [(self.short.is_some(), "short"),]
        );

        check_conflicts!(
            (self.no_long, "no_long"),
            [(self.long.is_some(), "long"),]
        );

        check_conflicts!(
            (self.no_short && self.no_long, "no_short and no_long"),
            [
                (self.inverse_arg, "inverse_arg"),
                (self.help.is_some(), "help"),
            ]
        );

        check_conflicts!(
            (self.no_file, "no_file"),
            [
                (self.file.is_some(), "file"),
                (self.section.is_some(), "section"),
            ]
        );

        Ok(())
    }

    fn parse_default(&self) -> Result<proc_macro2::TokenStream> {
        // TODO: get values inside Option<T>
        Ok(match self.default.to_owned() {
            Some(expr) => {
                let expr = syn::parse_str::<Expr>(&expr)?;
                quote! { #expr }
            }
            None => quote! { ::std::default::Default::default() },
        }.into())
    }

    fn parse_file(&self) -> Option<OptFileData> {
        if self.no_file {
            None
        } else {
            Some(OptFileData {
                section: self
                    .section
                    .clone()
                    .unwrap_or(String::from("Defaults")),
            })
        }
    }

    fn parse_arg(&self) -> Result<Option<OptArgData>> {
        // The long or short values may be empty, meaning that the
        // value should be converted from the field name.
        if self.no_long && self.no_short {
            Ok(None)
        } else {
            let get_ident = || self.ident.clone().unwrap().to_string();

            let long: Option<String> = if self.no_long {
                None
            } else {
                Some(self.long.to_owned().unwrap_or_else(get_ident))
            };

            let short = if self.no_short {
                None
            } else {
                match self.short.to_owned() {
                    Some(s) => {
                        // If the user provides the short name, this makes
                        // sure it's a single character.
                        let mut chars = s.chars();
                        let first = chars.next();
                        let second = chars.next();

                        match (first, second) {
                            (Some(ch), None) => Some(ch.to_string()),
                            _ => {
                                return Err(Error {
                                    span: self.ident.span(),
                                    kind: ErrorKind::Parse(
                                        String::from("short argument can't \
                                        be longer than one character")
                                    )
                                })
                            }
                        }
                    }
                    None => {
                        // Otherwise, the short name is obtained from the
                        // identifier, which must be at least a character
                        // long, so `unwrap()` is used.
                        Some(get_ident().chars().nth(0).unwrap().to_string())
                    }
                }
            };

            Ok(Some(OptArgData {
                long,
                short,
                help: self.help.clone(),
                inverse: self.inverse_arg,
            }))
        }
    }
}

impl Opt {
    pub fn parse(field: Field) -> Result<Opt> {
        let data = BasicOptAttrs::from_field(&field)?;
        data.check_conflicts()?;

        Ok(Opt {
            default: data.parse_default()?,
            file: data.parse_file(),
            arg: data.parse_arg()?,
            name: data.ident.unwrap(),
            ty: data.ty,
        })
    }
}
