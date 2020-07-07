extern crate darling;
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
    // TODO remove unwrap
    let options: Vec<Opt> = fields.named.iter().map(|f| Opt::parse(f).unwrap()).collect();
    let new_fields = options.iter().map(|opt| opt.parse_field_init());
    let new_args = options.iter().map(|opt| opt.parse_arg_init());

    Ok(quote! {
        impl StructConf for #name {
            fn new<'a>(app: clap::App<'a, 'a>) -> #name {
                let args = parse_args(app);
                let file = parse_config_file(args.value_of("config_file"));
                #name {
                    #(#new_fields,)*
                }
            }

            fn to_file(&self) {
                // TODO
            }

            fn refresh_file(&mut self) {
                // TODO
            }
        }

        // TODO: these functions should go inside the struct itself or
        // similars. Its visibility should be reduced.
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

        fn parse_args<'a>(app: clap::App<'a, 'a>) -> clap::ArgMatches<'a> {
            app.args(&[
                #(#new_args,)*
            ]).get_matches()
        }
    }.into())
}
