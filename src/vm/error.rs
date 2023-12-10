use crate::cc_parser::ast::{Span, Spanned};
use codespan_reporting::{
    diagnostic::{Diagnostic, Label},
    files::SimpleFile,
    term,
};
use std::io;
use termcolor::WriteColor;
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

impl AsDiagnostic for Error {
    fn as_diagnostic(&self, span: &Span) -> Diagnostic<()> {
        match self {
            Error::AttributeError(e) => e.as_diagnostic(span),
            Error::IoError(e) => e.as_diagnostic(span),
            Error::NameError(e) => e.as_diagnostic(span),
            Error::OverflowError(e) => e.as_diagnostic(span),
            Error::SyntaxError(e) => e.as_diagnostic(span),
            Error::TypeError(e) => e.as_diagnostic(span),
            Error::IndexError(e) => e.as_diagnostic(span),
            Error::ArrayError(e) => e.as_diagnostic(span),
        }
    }
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
    #[error("cannot define more than 256 local variables in a function")]
    TooManyLocals,
    #[error("jump body is too large")]
    StackOverflow,
    #[error("cannot use more than 256 arguments in a function")]
    TooManyConstants,
    #[error("cannot define more than 256 local variables in a function")]
    TooManyUpvalues,
    #[error("cannot use more than 256 arguments in a function")]
    TooManyArguments,

    #[error("jump body is too large")]
    JumpTooLarge,
}

impl AsDiagnostic for OverflowError {
    fn as_diagnostic(&self, span: &Span) -> Diagnostic<()> {
        Diagnostic::error()
            .with_code("AttributeError")
            .with_message(self.to_string())
            .with_labels(vec![Label::primary((), span.clone())])
    }
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

impl AsDiagnostic for NameError {
    fn as_diagnostic(&self, span: &Span) -> Diagnostic<()> {
        Diagnostic::error()
            .with_code("NameError")
            .with_message(self.to_string())
            .with_labels(vec![Label::primary((), span.clone())])
    }
}

#[derive(Debug, Error, Eq, PartialEq)]
pub enum IoError {}

impl AsDiagnostic for IoError {
    fn as_diagnostic(&self, span: &Span) -> Diagnostic<()> {
        Diagnostic::error()
            .with_code("IoError")
            .with_message(self.to_string())
            .with_labels(vec![Label::primary((), span.clone())])
    }
}

#[derive(Debug, Error, Eq, PartialEq)]
pub enum SyntaxError {
    #[error("param must have type: {name}")]
    ParamMustHaveType { name: String },
    #[error("return outside function")]
    ReturnOutsideFunction,
    #[error("extraneous input: {token:?}")]
    ExtraToken { token: String },
    #[error("invalid input")]
    InvalidToken,
    #[error(r#""self" used outside class"#)]
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

impl AsDiagnostic for SyntaxError {
    fn as_diagnostic(&self, span: &Span) -> Diagnostic<()> {
        Diagnostic::error()
            .with_code("SyntaxError")
            .with_message(self.to_string())
            .with_labels(vec![Label::primary((), span.clone())])
    }
}

#[derive(Debug, Error, Eq, PartialEq)]
pub enum ArrayError {
    #[error("extend method only accepts array type, got {type_}")]
    ExtendMethodOnlyAcceptsArray { type_: String },
}

impl AsDiagnostic for ArrayError {
    fn as_diagnostic(&self, span: &Span) -> Diagnostic<()> {
        Diagnostic::error()
            .with_code("ArrayError")
            .with_message(self.to_string())
            .with_labels(vec![Label::primary((), span.clone())])
    }
}

#[derive(Debug, Error, Eq, PartialEq)]
pub enum TypeError {
    #[error("cannot cast into int from {type_}")]
    NotNumber { type_: String },

    #[error("array type must be {expected:?}, got {actual:?}")]
    ArrayValueTypeMismatch { expected: String, actual: String },

    #[error("array is not indexable type: {type_}")]
    NotIndexable { type_: String },

    #[error("array index must be int")]
    ArrayIndexMustBeInt,

    #[error("array type must be {expected:?}, got {actual:?}")]
    ArrayTypeMismatch { expected: String, actual: String },

    #[error("unsupported operand for string: {op} ")]
    UnSupportedOperandforString { op: String },

    #[error("binary op sides must be same: {lhs} {op} {rhs}")]
    BinaryOpSidesMustBeSame {
        lhs: String,
        rhs: String,
        op: String,
    },

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

    #[error("unsupported operand type for {op}: {lt_type:?} and {rt_type:?}")]
    UnsupportedOperandInfix {
        op: String,
        lt_type: String,
        rt_type: String,
    },

    #[error("loop must be boolean")]
    LoopMustBeBoolean,

    #[error("function must not return a value")]
    FunctionMustNotReturnValue,

    #[error("initializer must not return a value")]
    InitializerMustNotReturnValue,

    #[error("return type must be {expected:?}, got {actual:?}")]
    ReturnTypeMismatch { expected: String, actual: String },

    #[error("variable type must be {expected:?}, got {actual:?}")]
    VariableTypeMismatch { expected: String, actual: String },

    #[error("cannot call {type_}")]
    NotCallable { type_: String },
}

impl AsDiagnostic for TypeError {
    fn as_diagnostic(&self, span: &Span) -> Diagnostic<()> {
        Diagnostic::error()
            .with_code("TypeError")
            .with_message(self.to_string())
            .with_labels(vec![Label::primary((), span.clone())])
    }
}

#[derive(Debug, Error, Eq, PartialEq)]
pub enum AttributeError {
    #[error("no such attribute: {name} on {type_}")]
    NoSuchAttribute { name: String, type_: String },
}

impl AsDiagnostic for AttributeError {
    fn as_diagnostic(&self, span: &Span) -> Diagnostic<()> {
        Diagnostic::error()
            .with_code("AttributeError")
            .with_message(self.to_string())
            .with_labels(vec![Label::primary((), span.clone())])
    }
}

#[derive(Debug, Error, Eq, PartialEq)]
pub enum IndexError {
    #[error("index out of range index: {index}, len: {len}")]
    IndexOutOfRange { index: usize, len: usize },
}

impl AsDiagnostic for IndexError {
    fn as_diagnostic(&self, span: &Span) -> Diagnostic<()> {
        Diagnostic::error()
            .with_code("IndexError")
            .with_message(self.to_string())
            .with_labels(vec![Label::primary((), span.clone())])
    }
}

trait AsDiagnostic {
    fn as_diagnostic(&self, span: &Span) -> Diagnostic<()>;
}

fn one_of(tokens: &[String]) -> String {
    let (token_last, tokens) = match tokens.split_last() {
        Some((token_last, &[])) => return token_last.to_string(),
        Some((token_last, tokens)) => (token_last, tokens),
        None => return "nothing".to_string(),
    };

    let mut output = String::new();
    for token in tokens {
        output.push_str(token);
        output.push_str(", ");
    }
    output.push_str("or ");
    output.push_str(token_last);
    output
}

/// For repl
pub fn report_errors(writer: &mut impl io::Write, source: &str, errors: &[ErrorS]) {
    let mut buffer = termcolor::Buffer::ansi();
    for err in errors {
        report_error(&mut buffer, source, err);
    }
    writer
        .write_all(buffer.as_slice())
        .expect("failed to write to output");
}

pub fn report_error(writer: &mut impl WriteColor, source: &str, (error, span): &ErrorS) {
    let file = SimpleFile::new("<script>", source);
    let config = term::Config::default();
    let diagnostic = error.as_diagnostic(span);
    term::emit(writer, &config, &file, &diagnostic).expect("failed to write to output");
}
