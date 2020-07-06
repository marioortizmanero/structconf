extern crate proc_macro;
mod opt;

use crate::opt::Opt;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse::Error, Data, DataStruct, DeriveInput, FieldsNamed};

#[proc_macro_derive(StructConf, attributes(conf))]
pub fn derive_conf(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast: DeriveInput = syn::parse(input).unwrap();

    // Build the trait implementation
    let result: Result<TokenStream, Error> = match &ast.data {
        Data::Struct(DataStruct {
            fields: syn::Fields::Named(named),
            ..
        }) => impl_conf_macro(&ast, &named),
        _ => Err(Error::new(
            ast.ident.span(),
            "cannot derive Options for type",
        )),
    };

    match result {
        Ok(tokens) => tokens.into(),
        Err(e) => e.to_compile_error().into(),
    }
}

fn impl_conf_macro(
    input: &DeriveInput,
    fields: &FieldsNamed,
) -> Result<TokenStream, Error> {
    let name = &input.ident;
    let new_fields = fields.named.iter().map(|f| {
        let Opt { name, ty, default, file, .. } = Opt::parse(f);
        // If there's no data for the config file, it won't be taken into
        // account at all. Otherwise, the section in which the option resides
        // may be specified, having "Defaults" as the fallback.
        let check_conf = match file {
            Some(file) => {
                let s = file.section;
                quote! {
                    .or_else(|| file.get_from(Some(#s), stringify!(#name)))
                }
            },
            None => quote! {}
        };

        // This first check the value obtained by the argument parser. If that
        // fails, it will check the value from the config file.
        // If any of these existed, they are parsed into the required type
        // (this must succeed). Otherwise, it's assigned the default value.
        quote! {
            #name: args.value_of(stringify!(#name))
                #check_conf
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

    Ok(quote! {
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
    }.into())
}
