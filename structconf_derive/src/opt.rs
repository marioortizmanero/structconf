use darling::FromField;
use proc_macro2::Span;
use quote::quote;
use syn::{
    parse::Error, parse_str, spanned::Spanned, Expr, Field, Ident, Type,
};

pub struct OptArgData {
    // An argument option may contain a long name, a short name, or both.
    pub long: Option<String>,
    pub short: Option<String>,
    pub help: String,
    pub conf_file: bool,
    pub inverted: bool,
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

// TODO: improve compile-time error handling
// fn missing_attr(span: Span) -> Error {
//     Error::new(span, "No attribute 'conf' provided")
// }

fn unexpected_item(span: Span, item: &str, ty: &str) -> Error {
    Error::new(span, format!("unexpected '{}' in {} option", item, ty))
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
    arg_inverted: bool,
    #[darling(default)]
    conf_file: bool,
    #[darling(default)]
    no_file: bool,
    #[darling(default)]
    file: Option<String>,
    #[darling(default)]
    section: Option<String>,
}

impl BasicOptAttrs {
    fn check_conflicts(&self, span: Span) -> Result<(), Error> {
        // Given an original expression and a list of other expressions it
        // conflicts with, it returns an error in case both of them are
        // true. The macro makes this a bit less repetitive.
        macro_rules! check_conflicts {
            ($orig:expr, $others:expr) => {
                let (orig, orig_name) = $orig;
                if orig {
                    for (confl, confl_name) in $others.iter() {
                        if *confl {
                            return Err(Error::new(
                                span,
                                format!(
                                    "{} and {} are conflicting options.",
                                    orig_name, confl_name
                                ),
                            ));
                        }
                    }
                }
            };
        }

        // Empty fields may not use any other attribute
        check_conflicts!(
            (
                self.no_short && self.no_long && self.no_file,
                "`no_short`, `no_long` and `no_file`"
            ),
            [
                (self.default.is_some(), "`default`"),
                (self.long.is_some(), "`long`"),
                (self.short.is_some(), "`short`"),
                (self.help.is_some(), "`help`"),
                (self.conf_file, "`conf_file`"),
                (self.arg_inverted, "`arg_inverted`"),
                (self.file.is_some(), "`file`"),
                (self.section.is_some(), "`section`"),
            ]
        );

        check_conflicts!(
            (self.no_short, "`no_short`"),
            [(self.short.is_some(), "`short`"),]
        );

        check_conflicts!(
            (self.no_long, "`no_long`"),
            [(self.long.is_some(), "`long`"),]
        );

        check_conflicts!(
            (self.no_short && self.no_long, "`no_short` and `no_long`"),
            [
                (self.arg_inverted, "`arg_inverted`"),
                (self.help.is_some(), "`help`"),
                (self.conf_file, "`conf_file`"),
            ]
        );

        check_conflicts!(
            (self.no_file || self.conf_file, "`no_file` or `conf_file`"),
            [
                (self.file.is_some(), "`file`"),
                (self.section.is_some(), "`section`"),
            ]
        );

        Ok(())
    }

    fn parse_default(&self) -> proc_macro2::TokenStream {
        // TODO: get values inside Option<T>
        match self.default.to_owned() {
            Some(expr) => {
                // TODO: shouldn't unwrap
                let expr = parse_str::<Expr>(&expr).unwrap();
                quote! { #expr }
            }
            None => quote! { ::std::default::Default::default() },
        }
        .into()
    }

    // TODO: get span from Ident
    fn parse_file(&self) -> Option<OptFileData> {
        if self.no_file || self.conf_file {
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

    fn parse_arg(&self, span: Span) -> Result<Option<OptArgData>, Error> {
        // The long or short values may be empty, meaning that the
        // value should be converted from the field name.
        if self.no_long && self.no_short {
            Ok(None)
        } else {
            let get_ident = || self.ident.to_owned().unwrap().to_string();

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
                                return Err(Error::new(
                                    span,
                                    "short argument can't be longer than one \
                                character",
                                ))
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
                help: self.help.clone().unwrap_or_default(),
                inverted: self.arg_inverted,
                conf_file: self.conf_file,
            }))
        }
    }
}

impl Opt {
    pub fn parse(f: &Field) -> Result<Opt, Error> {
        // TODO: propagate instead of unwrap()
        let data = BasicOptAttrs::from_field(f).unwrap();
        let span = f.span();
        data.check_conflicts(span).unwrap();

        Ok(Opt {
            name: data.ident.clone().unwrap(),
            ty: data.ty.clone(),
            default: data.parse_default(),
            file: data.parse_file(),
            arg: data.parse_arg(span)?,
        })
    }
}
