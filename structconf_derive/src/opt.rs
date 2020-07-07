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
    pub fn get_long(name: &str) -> String {
        format!("--{}", name.replace("_", "-"))
    }

    pub fn get_short(name: &str) -> String {
        // Unwrap should never fail because empty names don't make sense
        format!("-{}", name.chars().next().unwrap())
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

    fn parse_file(
        span: Span,
        attr: &BasicOptAttrs,
    ) -> Result<Option<OptFileData>, Error> {
        // The option is only available in the config file if the
        // `file` parameter is used.
        if let Some(_) = attr.file {
            Ok(Some(OptFileData {
                // TODO: clone may be unnecessary
                section: attr
                    .section
                    .clone()
                    .unwrap_or(String::from("Defaults")),
            }))
        } else {
            if attr.section.is_some() {
                Err(unexpected_item(span, "section", "non-config-file"))
            } else {
                Ok(None)
            }
        }
    }

    fn parse_arg(
        span: Span,
        attr: &BasicOptAttrs,
    ) -> Result<Option<OptArgData>, Error> {
        // The long or short values may be empty, meaning that the
        // value should be converted from the field name.
        // TODO: Improve checks to include no_long and such
        // The option is only available in the argument parser if
        // a `long` or `short` name is indicated, or both.
        if attr.long.is_some() || attr.short.is_some() {
            Ok(Some(OptArgData {
                // TODO: clone may be unnecessary
                long: attr
                    .long
                    .clone()
                    .and_then(|x| Some(OptArgData::get_long(&x))),
                short: attr
                    .short
                    .clone()
                    .and_then(|x| Some(OptArgData::get_short(&x))),
                help: attr.help.clone().unwrap_or_default(),
            }))
        } else {
            if attr.help.is_some() {
                Err(unexpected_item(span, "help", "arg"))
            } else {
                Ok(None)
            }
        }
    }
}
