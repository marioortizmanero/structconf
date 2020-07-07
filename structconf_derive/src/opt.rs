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

impl OptArgData {
    pub fn get_long(name: Ident) -> String {
        format!("--{}", name.to_string().replace("_", "-"))
    }

    pub fn get_short(name: Ident) -> String {
        // Unwrap should never fail because empty names don't make sense
        format!("-{}", name.to_string().chars().next().unwrap())
    }
}

// TODO: may be unnecessary
// fn missing_attr(span: Span) -> Error {
//     Error::new(span, "No attribute 'conf' provided")
// }

fn unexpected_item(span: Span, item: &str, ty: &str) -> Error {
    Error::new(span, format!("unexpected '{}' in {} option", item, ty))
}

#[derive(Debug, FromField)]
#[darling(attributes(basic_opt))]
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

impl Opt {
    pub fn parse(f: &Field) -> Result<Opt, Error> {
        // TODO: not necessary
        // Obtains metadata from the single `#[conf(...)]` attribute, which
        // is mandatory to reduce ambiguity.
        // let attr = f.attrs.iter().find(|a| {
        //     a.path.segments.len() == 1 && a.path.segments[0].ident == "conf"
        // }).ok_or(missing_attr(f.span()))?;

        // TODO: propagate instead of unwrap()
        let data = BasicOptAttrs::from_field(f).unwrap();
        let span = f.span();

        Ok(Opt {
            name: data.ident.clone().unwrap(),
            ty: data.ty.clone(),
            default: Opt::parse_default(&data),
            file: Opt::parse_file(span, &data)?,
            arg: Opt::parse_arg(span, &data)?,
        })
    }

    fn parse_default(attr: &BasicOptAttrs) -> proc_macro2::TokenStream {
        // TODO: clone may be unnecessary
        // TODO: get values inside Option<T>
        match attr.default.clone() {
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
    fn parse_file(
        span: Span,
        attr: &BasicOptAttrs,
    ) -> Result<Option<OptFileData>, Error> {
        if attr.no_file || attr.conf_file {
            if attr.section.is_some() {
                Err(unexpected_item(span, "section", "non-config-file"))
            } else {
                Ok(None)
            }
        } else {
            Ok(Some(OptFileData {
                section: attr
                    .section
                    .clone()
                    .unwrap_or(String::from("Defaults")),
            }))
        }
    }

    fn parse_arg(
        span: Span,
        attr: &BasicOptAttrs,
    ) -> Result<Option<OptArgData>, Error> {
        // The long or short values may be empty, meaning that the
        // value should be converted from the field name.
        // TODO: check conflicting attributes, also checks for conf_file
        if attr.no_long && attr.no_short {
            if attr.help.is_some() {
                Err(unexpected_item(span, "help", "arg"))
            } else {
                Ok(None)
            }
        } else {
            // TODO clone may be unnecessary?
            let long: Option<String> = if attr.no_long {
                None
            } else {
                Some(attr.long.clone().unwrap_or_else(|| {
                    OptArgData::get_long(attr.ident.clone().unwrap())
                }))
            };

            let short = if attr.no_short {
                None
            } else {
                Some(attr.short.clone().unwrap_or_else(|| {
                    OptArgData::get_short(attr.ident.clone().unwrap())
                }))
            };

            Ok(Some(OptArgData {
                long,
                short,
                help: attr.help.clone().unwrap_or_default(),
                inverted: attr.arg_inverted,
                conf_file: attr.conf_file
            }))
        }
    }
}
