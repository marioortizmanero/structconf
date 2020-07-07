extern crate darling;
extern crate proc_macro;

mod opt;

use crate::opt::{Opt, OptArgData};

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
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
    let options: Vec<Opt> = fields
        .named
        .iter()
        .map(|f| Opt::parse(f).unwrap())
        .collect();
    let new_fields = parse_field_init(&options);
    let new_args = parse_args_init(&options);
    let to_file = parse_to_file(&options);
    let conf_path = parse_conf_path(&options);

    Ok(quote! {
        impl #name  {
            pub fn new<'a>(
                app: clap::App<'a, 'a>,
                default_conf_path: &str
            ) -> #name {
                let args = #name::parse_args(app);
                // TODO should be saved into 'self'
                let conf_path = #conf_path;
                let file = #name::parse_config_file(conf_path);

                #name {
                    #(#new_fields,)*
                }
            }

            pub fn to_file(&self) {
                let mut conf = ini::Ini::new();
                #(#to_file)*
                conf.write_to_file("...").unwrap();
            }

            pub fn refresh_file(&mut self) {
                // TODO
            }

            // TODO: these functions should go inside the struct itself or
            // similars. Its visibility should be reduced.
            fn parse_config_file(path: std::path::PathBuf) -> ini::Ini {
                // Checking that the config file exists, and creating it
                // otherwise.
                if !path.exists() {
                    std::fs::File::create(&path)
                        .expect("Could not create config file");
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
        }
    }
    .into())
}

fn parse_field_init(opts: &Vec<Opt>) -> Vec<TokenStream2> {
    opts.iter().map(|opt| {
        let Opt { name, ty, default, file, .. } = opt;
        // If there's no data for the config file, it won't be taken into
        // account at all. Otherwise, the section in which the option resides
        // may be specified, having "Defaults" as the fallback.
        let conf_file = file.as_ref().and_then(|f| {
            let section = f.section.as_str();
            Some(quote! {
                .or_else(|| file.get_from(Some(#section), stringify!(#name)))
            })
        }).unwrap_or_default();

        // This first check the value obtained by the argument parser. If that
        // fails, it will check the value from the config file.
        // If any of these existed, they are parsed into the required type
        // (this must succeed). Otherwise, it's assigned the default value.
        quote! {
            #name: args.value_of(stringify!(#name))
                #conf_file
                .and_then(|x| {
                    Some(x.parse::<#ty>().expect(&format!(
                        "The value for '{}' is invalid in the configuration: \
                        '{}'",
                        stringify!(#name),
                        x
                    )))
                })
                .unwrap_or(#default)
        }
    }).collect()
}

fn parse_args_init(opts: &Vec<Opt>) -> Vec<TokenStream2> {
    opts.iter()
        .map(|opt| {
            // In case it's not an argument, an empty TokenStream will be
            // returned.
            opt.arg
                .as_ref()
                .and_then(|data| {
                    let name = opt.name.to_string();
                    let OptArgData { long, short, help, .. } = data;

                    let long = long
                        .as_ref()
                        .and_then(|name| {
                            Some(quote! {
                                .long(#name)
                            })
                        })
                        .unwrap_or_default();

                    let short = short
                        .as_ref()
                        .and_then(|name| {
                            Some(quote! {
                                .short(#name)
                            })
                        })
                        .unwrap_or_default();

                    let init = quote! {
                        clap::Arg::with_name(#name)
                            #long
                            #short
                            .help(#help)
                    };

                    init.into()
                })
                .unwrap_or_default()
        })
        .collect()
}

fn parse_conf_path(opts: &Vec<Opt>) -> TokenStream2 {
    // Looks for an option with the `conf_file` attribute set.
    let opt = opts.iter().find(|a| {
        match &a.arg {
            Some(arg) => arg.conf_file,
            None => false
        }
    });

    let default = quote! {
        let mut path = dirs::config_dir()
            .expect("Couldn't find user config directory");
        path.extend(default_conf_path.split("/"));
        path
    };

    match opt {
        Some(Opt { name, .. }) => quote! {
            args.value_of(stringify!(#name))
                .and_then(std::path::PathBuf::from)
                .unwrap_or_else(|| { #default });
        },
        None => quote! {
            {
                #default
            };
        }
    }
}

fn parse_to_file(opts: &Vec<Opt>) -> Vec<TokenStream2> {
    opts.iter()
        .filter_map(|opt| {
            opt.file.as_ref().and_then(|file| {
                let name = opt.name.clone();
                let section = file.section.as_str();
                Some(quote! {
                    conf.with_section(Some(#section))
                        .set(stringify!(#name), self.#name.to_string());
                })
            })
        })
        .collect()
}
