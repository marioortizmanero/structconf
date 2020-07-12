use proc_macro2::Span;
use std::fmt;

pub type Result<T> = std::result::Result<T, Error>;

// The errors that may occur during the execution of the procedural macro.
pub struct Error {
    pub kind: ErrorKind,
    pub span: Span,
}

#[derive(Debug)]
pub enum ErrorKind {
    DeriveType(String),
    ConflictAttrs(String, String),
    ConflictIDs(String, String),
    Value(String, String),
    Parse(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use ErrorKind::*;

        match &self.kind {
            DeriveType(ty) => {
                write!(f, "Cannot #[derive(StructConf)] for the type `{}`, \
                StructConf is only available for named structs.", ty)
            }
            ConflictAttrs(opt1, opt2) => write!(
                f,
                "`{}` and `{}` are conflicting attributes, they cannot go \
                together in the same field.",
                opt1, opt2
            ),
            ConflictIDs(attr, val) => write!(
                f,
                "The `{}` attribute with value `{}` is repeated with another \
                field.",
                attr, val
            ),
            Value(opt, val) => {
                write!(f, "Invalid value for `{}`: {}", opt, val)
            }
            Parse(desc) => write!(f, "Couldn't parse attribute: {}", desc),
        }
    }
}

impl From<Error> for syn::Error {
    fn from(err: Error) -> Self {
        syn::Error::new(err.span, err.to_string())
    }
}

impl From<darling::Error> for Error {
    fn from(err: darling::Error) -> Self {
        Error {
            kind: ErrorKind::Parse(err.to_string()),
            span: Span::call_site(),
        }
    }
}

impl From<syn::Error> for Error {
    fn from(err: syn::Error) -> Self {
        Error {
            kind: ErrorKind::Parse(err.to_string()),
            span: err.span(),
        }
    }
}
