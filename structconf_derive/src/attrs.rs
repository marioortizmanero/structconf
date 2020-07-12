//! The basic structure for the field's data, containing all the available
//! attributes in `#[conf(...)]` and some additional data. This is first
//! obtained with the module `darling`, and then parsed into a higher level
//! structure defined later.

use crate::error::{Error, ErrorKind, Result};
use crate::opt::*;

use darling::FromField;
use syn::{spanned::Spanned, Field, Ident, Type, Path, TypePath};

// TODO rename to attrs
#[derive(FromField)]
#[darling(attributes(conf))]
pub struct BasicOptAttrs {
    pub ident: Option<Ident>,
    pub ty: Type,
    #[darling(skip)]
    pub is_option: bool,
    #[darling(skip)]
    pub takes_value: bool,
    #[darling(default)]
    pub default: Option<String>,
    #[darling(default)]
    pub no_long: bool,
    #[darling(default)]
    pub long: Option<String>,
    #[darling(default)]
    pub no_short: bool,
    #[darling(default)]
    pub short: Option<String>,
    #[darling(default)]
    pub help: Option<String>,
    #[darling(default)]
    pub inverse_arg: bool,
    #[darling(default)]
    pub no_file: bool,
    #[darling(default)]
    pub file: Option<String>,
    #[darling(default)]
    pub section: Option<String>,
}

impl BasicOptAttrs {
    /// Method to initialize a `BasicOptAttrs` completely, from the parsing
    /// done by `darling`, and some extra checks for conflicts and for the
    /// type. The `BasicOptAttrs::from_field` method generated by `darling`
    /// shouldn't be used by itself.
    pub fn init(field: Field) -> Result<BasicOptAttrs> {
        let attrs = BasicOptAttrs::from_field(&field)?;
        attrs.check_conflicts()?;
        let (ty, is_option) = attrs.get_type();

        Ok(BasicOptAttrs {
            ty,
            is_option,
            takes_value: attrs.takes_value,
            ..attrs
        })
    }

    fn check_conflicts(&self) -> Result<()> {
        // Given an original expression and a list of other expressions it
        // conflicts with, it returns an error in case both of them are true.
        // The macro makes this a bit less repetitive.
        macro_rules! check_conflicts {
            ($orig:expr, $others:expr) => {
                let (orig, orig_name) = $orig;
                if orig {
                    for (confl, confl_name) in $others.iter() {
                        if *confl {
                            return Err(Error {
                                span: self.ident.span(),
                                kind: ErrorKind::ConflictAttrs(
                                    orig_name.to_string(),
                                    confl_name.to_string(),
                                ),
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

    /// Returns the type of the option or the inner type in an `Option<T>`,
    /// and whether it was optional or not.
    fn get_type(&self) -> (Type, bool) {
        // Painfully obtaining the type `T` inside `Option<T>` for parsing.
        if let Type::Path(TypePath {
            path: Path { segments, .. },
            ..
        }) = &self.ty
        {
            if segments.len() == 1
                && segments.first().unwrap().ident == "Option"
            {
                let args = &segments.first().unwrap().arguments;
                use syn::{
                    AngleBracketedGenericArguments as Brackets,
                    GenericArgument::Type as InnerType,
                    PathArguments::AngleBracketed as PathAngles,
                };

                // Obtaining the type inside the `Option<T>`.
                if let PathAngles(Brackets { args, .. }) = args {
                    if let InnerType(ty) = args.first().unwrap() {
                        return (ty.clone(), true);
                    }
                }
            }
        }

        (self.ty.clone(), false)
    }

    pub fn takes_value(&self) -> bool {
        if let Type::Path(TypePath {
            path: Path { segments, .. },
            ..
        }) = &self.ty
        {
            if segments.len() == 1
                && segments.first().unwrap().ident == "bool"
            {
                return false;
            }
        }

        true
    }

    pub fn parse_file(&self) -> Option<OptFileData> {
        if self.no_file {
            None
        } else {
            Some(OptFileData {
                name: self
                    .file
                    .clone()
                    .unwrap_or(self.ident.as_ref().unwrap().to_string()),
                section: self
                    .section
                    .clone()
                    .unwrap_or(String::from("Defaults")),
            })
        }
    }

    pub fn parse_arg(&self) -> Result<Option<OptArgData>> {
        // The long or short values may be empty, meaning that the
        // value should be converted from the field name.
        if self.no_long && self.no_short {
            Ok(None)
        } else {
            let get_ident = || self.ident.clone().unwrap().to_string();

            let long: Option<String> = if self.no_long {
                None
            } else {
                let long = self.long.to_owned().unwrap_or_else(get_ident);
                Some(long.replace("_", "-"))
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
                                    kind: ErrorKind::Parse(String::from(
                                        "short argument can't \
                                        be longer than one character",
                                    )),
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

    pub fn parse_opt(&self) -> Result<&dyn Opt> {
        let base = OptBaseData {
            takes_value: self.takes_value(),
            is_option: self.is_option,
            default: self.default,
            name: self.ident.unwrap(),
            ty: self.ty,
        };
        let arg = self.parse_arg()?;
        let file = self.parse_file();

        if arg.is_none() && file.is_none() {
            Ok(&OptNone {
                base
            })
        } else if arg.is_some() && file.is_none() {
            Ok(&OptArg {
                base,
                arg: arg.unwrap(),
            })
        } else if arg.is_none() && file.is_some() {
            Ok(&OptFile {
                base,
                file: file.unwrap(),
            })
        } else {
            Ok(&OptBoth {
                base,
                arg: arg.unwrap(),
                file: file.unwrap()
            })
        }
    }
}