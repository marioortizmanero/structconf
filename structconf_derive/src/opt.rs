use quote::quote;
use syn::{Type, Field, Attribute, parse_str, Expr};

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
    pub name: proc_macro2::Ident,
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

impl Opt {
    pub fn parse(f: &Field) -> Opt {
        // Obtains metadata from the single `#[conf(...)]` attribute.
        let attr: Option<&Attribute> = f.attrs.iter().find(|a| {
            a.path.segments.len() == 1 && a.path.segments[0].ident == "conf"
        });

        let get_val = |name, weak| {
            attr.and_then(|attr| Opt::get_group_value(attr, name, weak))
        };

        Opt {
            // TODO: avoid cloning and investigate unwrap
            name: f.clone().ident.unwrap(),
            ty: f.clone().ty,
            default: {
                match get_val("default", false) {
                    Some(expr) => {
                        // TODO: shouldn't unwrap
                        let expr = parse_str::<Expr>(&expr).unwrap();
                        quote! { #expr }
                    }
                    None => quote! { ::std::default::Default::default() }
                }.into()
            },
            file: {
                // The option is only available in the config file if the
                // `file` parameter is used.
                if let Some(_) = get_val("file", true) {
                    Some(OptFileData {
                        section: get_val("section", false)
                            .unwrap_or(String::from("Defaults"))
                    })
                } else {
                    None
                }
            },
            arg: {
                // The long or short values may be empty, meaning that the
                // value should be converted from the field name.
                let long = get_val("long", true)
                    .and_then(|x| Some(OptArgData::get_long(&x)));
                let short = get_val("short", true)
                    .and_then(|x| Some(OptArgData::get_short(&x)));

                // The option is only available in the argument parser if
                // a `long` or `short` name is indicated, or both.
                if long.is_some() || short.is_some() {
                    Some(OptArgData {
                        long,
                        short,
                        help: get_val("help", false).unwrap_or_default(),
                    })
                } else {
                    None
                }
            },
        }
    }

    // Obtains the vlaue in an attribute with syntax `key = "value"`. In case
    // it's just `key` and `weak` is true, the returned value will be empty.
    pub fn get_group_value(
        attr: &Attribute,
        key: &str,
        weak: bool,
    ) -> Option<String> {
        if let Some(proc_macro2::TokenTree::Group(g)) =
            attr.tokens.clone().into_iter().next()
        {
            let mut tokens = g.stream().into_iter();
            Some(String::from("fck"))
        } else {
            None
        }
    }
}
