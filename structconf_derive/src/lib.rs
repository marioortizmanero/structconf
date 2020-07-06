extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse::Error, Data, DataStruct, DeriveInput, Field, FieldsNamed,
          Attribute, Type, parse_str, Expr};

// An argument option may contain a long name, a short name, or both.
struct OptArgData {
    long: Option<String>,
    short: Option<String>,
    help: String
}

struct OptFileData {
    section: String,
}

struct Opt {
    name: proc_macro2::Ident,
    ty: Type,
    default: Option<String>,
    file: Option<OptFileData>,
    arg: Option<OptArgData>
}

#[proc_macro_derive(StructConf, attributes(conf))]
pub fn derive_conf(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast: DeriveInput = syn::parse(input).unwrap();

    // Build the trait implementation
    let result: Result<TokenStream, Error> = match &ast.data {
        Data::Struct(DataStruct{fields: syn::Fields::Named(named), ..}) => impl_conf_macro(&ast, &named),
        _ => Err(Error::new(ast.ident.span(), "cannot derive Options for type")),
    };

    match result {
        Ok(tokens) => tokens.into(),
        Err(e) => e.to_compile_error().into()
    }
}

fn impl_conf_macro(input: &DeriveInput, fields: &FieldsNamed) -> Result<TokenStream, Error> {
    let name = &input.ident;
    let new_fields = fields.named.iter().map(|f| {
        let data = parse_conf(f);
        let name = data.name;
        // TODO: shouldn't unwrap
        let section = data.file.and_then(|x| Some(x.section)).unwrap_or(String::from("Defaults"));
        let ty = data.ty;
        let default = match data.default {
            Some(expr) => {
                // TODO: shouldn't unwrap
                let expr = parse_str::<Expr>(&expr).unwrap();
                quote! { #expr }
            }
            None => quote! { ::std::default::Default::default() }
        };

        quote! {
            #name: args.value_of(stringify!(#name))
                .or_else(|| file.get_from(Some(#section), stringify!(#name)))
                .and_then(|x| {
                    Some(x.parse::<#ty>().expect(&format!(
                        "The value for '{}' is invalid in the configuration: '{}'",
                        stringify!(#name),
                        x
                    )))
                })
                .unwrap_or(#default)
        }
    });
    // println!("ORIGINAL : : : :{:#?}", &new_fields.collect::<Vec<_>>());
    // let new_fields = vec![quote! { debug: true }, quote! { value: 213 } ];
    // println!("DUMMY : : : :{:#?}", &new_fields);


    let gen = quote!{
        impl StructConf for #name {
            fn new() -> std::sync::RwLock<#name> {
                let args = parse_args();
                let file = parse_config_file(args.value_of("config_file"));
                std::sync::RwLock::new(#name {
                    #(#new_fields,)*
                })
            }

            fn to_file(&self) {
            }
        }

        fn parse_config_file(path_str: Option<&str>) -> ini::Ini {
            // The path is first obtained as a `String` for output and
            let path = match path_str {
                Some(path) => std::path::PathBuf::from(path),
                None => {
                    let mut path = dirs::config_dir()
                        .expect("Could not find user config directory");
                    path.extend(["vidify", "config.ini"].iter());
                    path
                }
            };

            // Checking that the config file exists, and creating it otherwise.
            if !path.exists() {
                std::fs::File::create(&path).expect("Could not create config file");
                println!(
                    "Created config file at {}",
                    path.to_str().expect(
                        "Invalid UTF-8 characters found in the config path"
                    )
                );
            }
            ini::Ini::load_from_file(path).unwrap()
        }

        fn parse_args<'a>() -> clap::ArgMatches<'a> {
            // Basic information about the program.
            let app = clap::App::new("vidify")
                .version(clap::crate_version!())
                .author(clap::crate_authors!());

            // All the available options as arguments.
            app.args(&[]).get_matches()
        }
    };

    Ok(gen.into())
}

fn parse_conf(f: &Field) -> Opt {
    // Obtains metadata from the single `#[conf(...)]` attribute.
    let attr: Option<&Attribute> = f.attrs.iter().find(|a| {
        a.path.segments.len() == 1 && a.path.segments[0].ident == "conf"
    });

    let get_val = |name, weak| attr.and_then(|attr| Opt::get_group_value(attr, name, weak));

    Opt {
        // TODO: avoid cloning and investigate unwrap
        name: f.clone().ident.unwrap(),
        ty: f.clone().ty,
        default: get_val("default", false),
        file: {
            let section = get_val("section", false);
            if section.is_none() {
                None
            } else {
                Some(OptFileData {
                    section: section.unwrap()
                })
            }
        },
        arg: {
            let long = get_val("long", true).and_then(|x| Some(OptArgData::get_long(&x)));
            let short = get_val("short", true).and_then(|x| Some(OptArgData::get_short(&x)));

            if long.is_none() && short.is_none() {
                None
            } else {
                Some(OptArgData {
                    long,
                    short,
                    help: get_val("help", false).unwrap_or(String::from(""))
                })
            }
        }
    }
}

impl OptArgData {
    fn get_long(name: &str) -> String {
        format!("--{}", name.replace("_", "-"))
    }

    fn get_short(name: &str) -> String {
        // Unwrap should never fail because empty names don't make sense
        format!("-{}", name.chars().next().unwrap())
    }
}

impl Opt {
    // Obtains the vlaue in an attribute with syntax `key = "value"`. In case
    // it's just `key` and `weak` is true, the returned value will be empty.
    fn get_group_value(attr: &Attribute, key: &str, weak: bool) -> Option<String> {
        // if let Some(proc_macro2::TokenTree::Group(g)) = attr.tts.clone().into_iter().next() {
            // let mut tokens = g.stream().into_iter();

            // Some(String::from("fck"))
        // } else {
            // None
        // }
        Some(String::from("true"))
    }
}
