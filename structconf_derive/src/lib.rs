extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse::Error, Data, DataStruct, DeriveInput, Fields};

struct ArgData {
    long: String,
    short: String,
    help: String
}

struct FileData {
    section: String,
}

struct Field {
    name: String,
    default: String,
    file: Option<FileData>,
    arg: Option<ArgData>
}

#[proc_macro_derive(StructConf, attributes(conf))]
pub fn derive_conf(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast: DeriveInput = syn::parse(input).unwrap();

    // Build the trait implementation
    let result: Result<TokenStream, Error> = match &ast.data {
        Data::Struct(DataStruct{fields, ..}) => impl_conf_macro(&ast, &fields),
        _ => Err(Error::new(ast.ident.span(), "cannot derive Options for type")),
    };

    match result {
        Ok(tokens) => tokens.into(),
        Err(e) => e.to_compile_error().into()
    }
}

fn impl_conf_macro(input: &DeriveInput, fields: &Fields) -> Result<TokenStream, Error> {
    // println!("HELLO THIS IS A TEST HEHE");

    // for attr in fields {
        // println!("{:?}", field.attrs[0].tokens);
        // for token in field.attrs[0].tokens.stream {
            // match token {
                // Ident => println!("key: {:?}", token.ident),
                // proc_macro::Punct{ch, ..} => if ch == '=' {
                    // continue
                // } else {
                    // panic!("wtf");
                // },
                // // proc_macro::Literal{Lit{symbol, ..}, ..} => println!("value: {:?}", symbol),
                // _ => continue
            // }
        // }
    // }

    let name = &input.ident;

    let gen = quote!{
        use std::sync::RwLock;

        impl StructConf for #name {
            fn new() -> RwLock<#name> {
                RwLock::new(#name {
                })
            }

            fn to_file(&self) {
            }
        }
    };

    Ok(gen.into())
}
