extern crate darling;
extern crate proc_macro;

mod error;
mod opt;

use crate::opt::Opt;
use crate::error::{Error, ErrorKind};

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{Data, DataStruct, DeriveInput, Fields, FieldsNamed};

#[proc_macro_derive(StructConf, attributes(conf))]
pub fn derive_conf(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast: DeriveInput = syn::parse(input).unwrap();

    // Build the trait implementation
    let result: Result<TokenStream, Error> = match &ast.data {
        Data::Struct(DataStruct {
            fields: Fields::Named(named),
            ..
        }) => impl_conf_macro(&ast, &named),
        Data::Struct(_) => Err(Error{
            kind: ErrorKind::DeriveType(String::from("unnamed struct")),
            span: ast.ident.span(),
        }),
        Data::Enum(_) => Err(Error{
            kind: ErrorKind::DeriveType(String::from("enum")),
            span: ast.ident.span(),
        }),
        Data::Union(_) => Err(Error{
            kind: ErrorKind::DeriveType(String::from("union")),
            span: ast.ident.span(),
        }),
    };

    match result {
        Ok(tokens) => tokens.into(),
        Err(e) => syn::Error::from(e).to_compile_error().into(),
    }
}

fn impl_conf_macro(
    input: &DeriveInput,
    fields: &FieldsNamed,
) -> Result<TokenStream, Error> {
    let name = &input.ident;
    let mut options = Vec::new();
    for field in fields.named.iter() {
        options.push(Opt::parse(field)?);
    };
    let new_fields = parse_field_init(&options);
    let new_args = parse_args_init(&options);
    let to_file = parse_to_file(&options);

    Ok(quote! {
        impl StructConf for #name {
            fn parse(
                app: ::clap::App,
                path: &str
            ) -> Result<#name, ::structconf::Error> where Self: Sized {
                let args = #name::parse_args(app);
                #name::parse_file(args, path)
            }

            fn parse_args<'a>(
                app: ::clap::App<'a, 'a>
            ) -> ::clap::ArgMatches<'a> {
                app.args(&[
                    #(#new_args,)*
                ]).get_matches()
            }

            fn parse_file(
                args: ::clap::ArgMatches,
                path: &str
            ) -> Result<#name, ::structconf::Error> where Self: Sized {
                // Checking that the config file exists, and creating it
                // otherwise.
                let path_wrap = ::std::path::Path::new(path);
                if !path_wrap.exists() {
                    ::std::fs::File::create(&path_wrap)?;
                    println!("Created config file at {}", path);
                }

                let file = ::ini::Ini::load_from_file(path)?;
                Ok(#name {
                    #(#new_fields,)*
                })
            }

            fn write_file(
                &self,
                path: &str
            ) -> Result<(), ::structconf::Error> {
                let mut conf = ::ini::Ini::new();
                #(#to_file)*
                conf.write_to_file(path)?;

                Ok(())
            }
        }
    }
    .into())
}

fn parse_field_init(opts: &Vec<Opt>) -> Vec<TokenStream2> {
    opts.iter().map(|opt| {
        let Opt { name, ty, default, file, .. } = opt;
        let mut init = quote! {
            #name: args.value_of(stringify!(#name))
        };

        // If there's no data for the config file, it won't be taken into
        // account at all. Otherwise, the section in which the option resides
        // may be specified, having "Defaults" as the fallback.
        if let Some(file) = &file {
            let section = file.section.as_str();
            init.extend(quote! {
                .or_else(|| file.get_from(Some(#section), stringify!(#name)))
            });
        };

        // This first check the value obtained by the argument parser. If that
        // fails, it will check the value from the config file.
        // If any of these existed, they are parsed into the required type
        // (this must succeed). Otherwise, it's assigned the default value.
        init.extend(quote! {
                .and_then(|x| {
                    Some(x.parse::<#ty>().expect(&format!(
                        "The value for '{}' is invalid in the configuration: \
                        '{}'",
                        stringify!(#name),
                        x
                    )))
                })
                .unwrap_or(#default)
        });

        init
    }).collect()
}

fn parse_args_init(opts: &Vec<Opt>) -> Vec<TokenStream2> {
    opts.iter()
        .filter_map(|opt| {
            // In case it's not an argument, an empty TokenStream will be
            // returned.
            opt.arg
                .as_ref()
                .and_then(|data| {
                    let name = opt.name.to_string();
                    let mut init = quote! {
                        ::clap::Arg::with_name(#name)
                    };

                    if let Some(help) = &data.help {
                        init.extend(quote! {
                            .help(#help)
                        });
                    }

                    if let Some(long) = &data.long {
                        init.extend(quote! {
                            .long(#long)
                        });
                    }

                    if let Some(short) = &data.short {
                        init.extend(quote! {
                            .short(#short)
                        });
                    }

                    Some(init)
                })
        })
        .collect()
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
