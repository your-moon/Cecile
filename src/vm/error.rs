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
    #[error("IoError: {0}")]
    IoError(IoError),
    #[error("NameError {0}")]
    NameError(NameError),
    #[error("OverflowError {0}")]
    OverflowError(OverflowError),
    #[error("AttributeError {0}")]
    AttributeError(AttributeError),
    #[error("IndexError {0}")]
    IndexError(IndexError),
    #[error("ArrayError {0}")]
    ArrayError(ArrayError),
}

macro_rules! impl_from_error {
    ($($error:tt),+) => {$(
        impl From<$error> for Error {
            fn from(e: $error) -> Self {
                Error::$error(e)
            }
        }
    )+};
}

impl_from_error!(
    IoError,
    NameError,
    OverflowError,
    SyntaxError,
    TypeError,
    AttributeError,
    IndexError,
    ArrayError
);

#[derive(Debug, Error, Eq, PartialEq)]
pub enum OverflowError {
    #[error("jump body is too large")]
    StackOverflow,
    #[error("cannot use more than 256 arguments in a function")]
    TooManyConstants,
    #[error("cannot define more than 256 local variables in a function")]
    TooManyUpvalues,
    #[error("cannot use more than 256 arguments in a function")]
    TooManyArguments,
    #[error("loop body is too large")]
    LoopTooLarge,

    #[error("jump body is too large")]
    JumpTooLarge,
}

#[derive(Debug, Error, Eq, PartialEq)]
pub enum NameError {
    #[error("array has no field {name}")]
    ArrayHasNoField { name: String },

    #[error("struct is not in scope")]
    StructNotInScope,
    #[error("can't access local variable {name} in its own initializer")]
    AccessInsideInitializer { name: String },

    #[error("struct {name} has no field {name}")]
    StructFieldNotFound { name: String, struct_name: String },

    #[error("struct {name} is not defined")]
    StructNameNotFound { name: String },

    #[error("struct {struct_name} has no method {name}")]
    StructMethodNotFound { name: String, struct_name: String },

    #[error("struct {struct_name} has no method or field {name}")]
    StructMethodOrFieldNotFound { name: String, struct_name: String },

    #[error("identifier {name} is not defined")]
    IdentifierNotDefined { name: String },

    #[error("struct cannot inherit from itself")]
    StructInheritFromItSelf { name: String },

    #[error("struct {name} has no super class")]
    StructHasNoSuper { name: String },

    #[error("variable {name} is not defined")]
    VariableNameNotFound { name: String },
}

#[derive(Debug, Error, Eq, PartialEq)]
pub enum IoError {}

#[derive(Debug, Error, Eq, PartialEq)]
pub enum SyntaxError {
    #[error("extraneous input: {token:?}")]
    ExtraToken { token: String },
    #[error("invalid input")]
    InvalidToken,
    #[error(r#"init() should not return a value"#)]
    ReturnInInitializer,
    #[error(r#""this" used outside class"#)]
    SelfOutsideClass,
    #[error("unexpected input")]
    UnexpectedInput { token: String },
    #[error("unexpected end of file")]
    UnrecognizedEOF { expected: Vec<String> },
    #[error("unexpected {token:?}")]
    UnrecognizedToken {
        token: String,
        expected: Vec<String>,
    },

    #[error("already declared in this scope: {name}")]
    AlreadyDeclared { name: String },
}

#[derive(Debug, Error, Eq, PartialEq)]
pub enum ArrayError {
    #[error("extend method only accepts array type, got {type_}")]
    ExtendMethodOnlyAcceptsArray { type_: String },
}

#[derive(Debug, Error, Eq, PartialEq)]
pub enum TypeError {
    #[error("array type must be {expected:?}, got {actual:?}")]
    ArrayValueTypeMismatch { expected: String, actual: String },

    #[error("array is not indexable type: {type_}")]
    NotIndexable { type_: String },

    #[error("array index must be int")]
    ArrayIndexMustBeInt,

    #[error("array type must be {expected:?}, got {actual:?}")]
    ArrayTypeMismatch { expected: String, actual: String },

    #[error("cannot set expected type {expected:?}, got {actual:?}")]
    SetTypeMisMatch { expected: String, actual: String },

    #[error("arity expected {expected:?}, got {actual:?} name: {name:?}")]
    ArityMisMatch {
        expected: usize,
        actual: usize,
        name: String,
    },

    #[error("condition must be boolean")]
    CondMustbeBoolean,

    #[error("unsupported operand type for {op}: {rt_type:?}")]
    UnsupportedOperandPrefix { op: String, rt_type: String },

    #[error("unsupported operand type for {op}: {lt_type:?} and {rt_type:?}")]
    UnsupportedOperandInfix {
        op: String,
        lt_type: String,
        rt_type: String,
    },

    #[error("loop must be boolean")]
    LoopMustBeBoolean,

    #[error("return type must be nil")]
    ReturnTypeMustNotReturnValue,

    #[error("initializer must not return a value")]
    InitializerMustNotReturnValue,

    #[error("return type must be {expected:?}, got {actual:?}")]
    ReturnTypeMismatch { expected: String, actual: String },

    #[error("variable type must be {expected:?}, got {actual:?}")]
    VariableTypeMismatch { expected: String, actual: String },

    #[error("cannot call {type_}")]
    NotCallable { type_: String },
}

#[derive(Debug, Error, Eq, PartialEq)]
pub enum AttributeError {
    #[error("no such attribute: {name} on {type_}")]
    NoSuchAttribute { name: String, type_: String },
}

#[derive(Debug, Error, Eq, PartialEq)]
pub enum IndexError {
    #[error("index out of range index: {index}, len: {len}")]
    IndexOutOfRange { index: usize, len: usize },
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
