use std::default;

use crate::cc_parser::ast::Spanned;
use thiserror::Error;

pub type Result<T, E = ErrorS> = std::result::Result<T, E>;
pub type ErrorS = Spanned<Error>;

#[derive(Debug, PartialEq, Error)]
pub enum Error {
    #[error("invalid input")]
    SyntaxError(SyntaxError),
    #[error("TypeError: {0}")]
    TypeError(TypeError),
}

#[derive(Debug, Error, Eq, PartialEq)]
pub enum SyntaxError {
    #[error("extraneous input: {token:?}")]
    ExtraToken { token: String },
    #[error("invalid input")]
    InvalidToken,
    #[error(r#"init() should not return a value"#)]
    ReturnInInitializer,
    #[error(r#""return" used outside function"#)]
    ReturnOutsideFunction,
    #[error(r#""super" used outside class"#)]
    SuperOutsideClass,
    #[error(r#""super" used in class without a superclass"#)]
    SuperWithoutSuperclass,
    #[error(r#""this" used outside class"#)]
    ThisOutsideClass,
    #[error("unexpected input")]
    UnexpectedInput { token: String },
    #[error("unexpected end of file")]
    UnrecognizedEOF { expected: Vec<String> },
    #[error("unexpected {token:?}")]
    UnrecognizedToken {
        token: String,
        expected: Vec<String>,
    },
    #[error("unterminated string")]
    UnterminatedString,
}

#[derive(Debug, Error, Eq, PartialEq)]
pub enum TypeError {
    #[error("unsupported operand type for {op}: {rt_type:?}")]
    UnsupportedOperandPrefix { op: String, rt_type: String },
    #[error("loop must be boolean")]
    LoopMustBeBoolean,
    #[error("if must be boolean")]
    IfMustBeBoolean,

    #[error("return type must be nil")]
    ReturnTypeMustBeNil,

    #[error("return type must be {expected:?}, got {actual:?}")]
    ReturnTypeMismatch { expected: String, actual: String },
}

//
// #[derive(Debug, Error, Eq, PartialEq)]
// pub enum Error {
//     #[error("AttributeError: {0}")]
//     AttributeError(AttributeError),
//     #[error("IOError: {0}")]
//     IoError(IoError),
//     #[error("NameError: {0}")]
//     NameError(NameError),
//     #[error("OverflowError: {0}")]
//     OverflowError(OverflowError),
//     #[error("SyntaxError: {0}")]
//     SyntaxError(SyntaxError),
//     #[error("TypeError: {0}")]
//     TypeError(TypeError),
// }
