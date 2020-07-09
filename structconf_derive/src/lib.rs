extern crate darling;
extern crate proc_macro;

mod error;
mod opt;

use crate::opt::{Opt, OptArgData, OptFileData};
use crate::error::{Error, ErrorKind, Result};

use proc_macro::TokenStream;
use proc_macro2::{Ident, TokenStream as TokenStream2};
use quote::quote;
use syn::{Data, DataStruct, DeriveInput, Fields, FieldsNamed};

#[proc_macro_derive(StructConf, attributes(conf))]
pub fn derive_conf(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast: DeriveInput = syn::parse(input).unwrap();
    let name = &ast.ident;

    // Build the trait implementation
    let result: Result<TokenStream> = match ast.data {
        Data::Struct(DataStruct {
            fields: Fields::Named(named_fields),
            ..
        }) => impl_conf_macro(name, named_fields),
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
    name: &Ident,
    fields: FieldsNamed,
) -> Result<TokenStream> {
    let mut options = Vec::new();
    for field in fields.named.into_iter() {
        options.push(Opt::parse(field)?);
    }
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

/// This will return a tokenstream for each field that has to be initialized,
/// depending on whether the option is an argument, a config file option,
/// none, or both:
///
/// ```rust
/// field: {
///     // May go through no branches, branch A, branch B, or both branches.
///     let mut opt: Option<String> = None;
/// 
///     // Branch A (argument)
///     opt = arg.value_of(...);
/// 
///     // Branch B (config file)
///     if let None = opt {
///         opt = file.get_from(...);
///     }
/// 
///     match opt {
///         // Branch A, B, or both
///         Some(opt) => {
///             // Parse `opt`
///             let opt = opt.parse()?;
///         
///             // Branch A, if `inverse_arg`, after parsing. `inverse_arg`
///             // may only be true if it's an argument.
///             if arg.value_of(...).is_some() {
///                 opt = !opt;
///             }
///         },
///         // None
///         None => default
///     }
/// }
/// ```
///
/// This is intended to be as flexible as possible, and to avoid repetition
/// in the code, so it may not be optimized for some cases. For example, if
/// the option is neither a config file option nor an argument, the `match`
/// block is useless because it's obvious it's going to go through the `None`
/// branch. But these cases are easy to optimize by the compiler, so it's not
/// important.
///
/// Methods like `unwrap_or` can't be used here because this has to be able
/// to propagate errors with `?`.
fn parse_field_init(opts: &Vec<Opt>) -> Vec<TokenStream2> {
    opts.iter().map(|opt| {
        let Opt { name, ty, default, file, arg, .. } = opt;

        let mut value = quote! {
            let mut opt: Option<&str> = None;
        };

        let mut invert_opt = TokenStream2::new();
        if let Some(OptArgData { inverse, .. }) = arg {
            value.extend(quote! {
                opt = args.value_of(stringify!(#name));
            });

            if *inverse {
                invert_opt.extend(quote! {
                    if args.value_of(stringify!(#name)).is_some() {
                        opt = !opt;
                    }
                });
            }
        }

        if let Some(OptFileData { section, .. }) = file {
            value.extend(quote! {
                if let None = opt {
                    opt = file.get_from(Some(#section), stringify!(#name));
                }
            });
        }

        value.extend(quote! {
            match opt {
                Some(opt) => {
                    let mut opt = opt
                        .parse::<#ty>()
                        .map_err(|e| {
                            ::structconf::Error::Parse(e.to_string())
                        })?;
                    #invert_opt
                    opt
                },
                None => #default
            }
        });

        quote! {
            #name: { #value }
        }
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
