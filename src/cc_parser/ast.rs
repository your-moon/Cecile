use std::fmt::{self, Display, Formatter};
use std::ops::Range;

use crate::vm::object::ObjectType;
pub type Spanned<T> = (T, Span);
pub type Span = Range<usize>;

pub type ExprS = Spanned<Expression>;
pub type StatementS = Spanned<Statement>;

#[derive(Debug, Default)]
pub struct Program {
    pub statements: Vec<StatementS>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Statement {
    Expression(StatementExpr),
    Block(StatementBlock),
    Print(StatementPrint),
    PrintLn(StatementPrintLn),
    Return(StatementReturn),
    Var(StatementVar),
    If(Box<StatementIf>),
    While(Box<StatementWhile>),
    For(Box<StatementFor>),
    Fun(StatementFun),
    Struct(StatementStruct),
    Impl(StatementImpl),
    Error,
}
impl Statement {
    pub fn as_fun(&self) -> Option<StatementFun> {
        match self {
            Statement::Fun(fun) => Some(fun.clone()),
            _ => None,
        }
    }

    pub fn is_fun(&self) -> bool {
        self.as_fun().is_some()
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct StatementImpl {
    pub name: String,
    pub super_: Option<ExprS>,
    pub methods: Vec<Spanned<Statement>>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct StatementStruct {
    pub name: String,
    pub fields: Vec<Field>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Field {
    pub name: String,
    pub type_: Type,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ExprCreateStructField {
    pub name: String,
    pub value: ExprS,
}

#[derive(Clone, Debug, PartialEq)]
pub struct StatementFun {
    pub name: String,
    pub params: Vec<(String, Option<Type>)>,
    pub body: StatementBlock,
    pub return_type: Option<Type>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct StatementFor {
    pub init: Option<StatementS>,
    pub cond: Option<ExprS>,
    pub update: Option<ExprS>,
    pub body: StatementS,
}

#[derive(Clone, Debug, PartialEq)]
pub struct StatementWhile {
    pub cond: ExprS,
    pub body: StatementS,
}

#[derive(Clone, Debug, PartialEq)]
pub struct StatementIf {
    pub cond: ExprS,
    pub then_branch: StatementS,
    pub else_branch: Option<StatementS>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct StatementVar {
    pub var: Var,
    pub value: Option<ExprS>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct StatementReturn {
    pub value: Option<ExprS>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct StatementPrintLn {
    pub value: ExprS,
}

#[derive(Clone, Debug, PartialEq)]
pub struct StatementPrint {
    pub value: ExprS,
}

#[derive(Clone, Debug, PartialEq)]
pub struct StatementBlock {
    pub statements: Vec<StatementS>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct StatementExpr {
    pub expr: ExprS,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Expression {
    Infix(Box<ExprInfix>),
    Prefix(Box<ExprPrefix>),
    Assign(Box<ExprAssign>),
    ArrayAccessAssign(Box<ExprArrayAccessAssign>),
    Call(Box<ExprCall>),
    Literal(ExprLiteral),
    Var(ExprVar),
    Get(Box<ExprGet>),
    Set(Box<ExprSet>),
    Super(ExprSuper),
    Array(Box<ExprArray>),
    ArrayAccess(Box<ExprArrayAccess>),
    Struct(Box<ExprStruct>),
}

impl Expression {
    pub fn as_var(&self) -> Option<ExprVar> {
        match self {
            Expression::Var(var) => Some(var.clone()),
            _ => None,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ExprStruct {
    pub name: String,
    pub fields: Vec<ExprSingleField>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ExprSingleField {
    pub name: String,
    pub value: ExprS,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ExprArrayAccess {
    pub array: ExprS,
    pub index: ExprS,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ExprArray {
    pub elements: Vec<ExprS>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ExprSuper {
    pub super_: Var,
    pub name: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ExprSet {
    pub object: ExprS,
    pub name: String,
    pub value: ExprS,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ExprGet {
    pub object: ExprS,
    pub name: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ExprCall {
    pub callee: ExprS,
    pub args: Vec<ExprS>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum ExprLiteral {
    Bool(bool),
    Nil,
    Number(f64),
    String(String),
}

#[derive(Clone, Debug, PartialEq)]
pub struct ExprArrayAccessAssign {
    pub array: ExprS,
    pub index: ExprS,
    pub value: ExprS,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ExprAssign {
    pub lhs: Var,
    pub rhs: ExprS,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ExprInfix {
    pub lhs: ExprS,
    pub op: OpInfix,
    pub rhs: ExprS,
}

#[derive(Clone, Debug, PartialEq)]
pub enum OpInfix {
    Add,
    Sub,
    Mul,
    Div,
    Less,
    Modulo,
    LessEqual,
    Greater,
    GreaterEqual,
    Equal,
    NotEqual,
    LogicAnd,
    LogicOr,
}

impl Display for OpInfix {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let op = match self {
            OpInfix::Modulo => "%",
            OpInfix::Add => "+",
            OpInfix::Sub => "-",
            OpInfix::Mul => "*",
            OpInfix::Div => "/",
            OpInfix::Less => "<",
            OpInfix::LessEqual => "<=",
            OpInfix::Greater => ">",
            OpInfix::GreaterEqual => ">=",
            OpInfix::Equal => "==",
            OpInfix::NotEqual => "!=",
            OpInfix::LogicAnd => "and",
            OpInfix::LogicOr => "or",
        };
        write!(f, "{op}")
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ExprPrefix {
    pub op: OpPrefix,
    pub rt: ExprS,
}

#[derive(Clone, Debug, PartialEq)]
pub enum OpPrefix {
    Negate,
    Not,
}

impl Display for OpPrefix {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let op = match self {
            OpPrefix::Negate => "-",
            OpPrefix::Not => "!",
        };
        write!(f, "{op}")
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ExprVar {
    pub var: Var,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Var {
    pub name: String,
    pub type_: Option<Type>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Fn {
    pub return_type: Box<Option<Type>>,
}

#[derive(Clone, Debug, PartialEq, Default, Eq)]
pub enum Type {
    Struct(String),
    Instance(String),
    Fn(Fn),
    Nil,
    String,
    Bool,
    Int,
    #[default]
    UnInitialized,
    Array(Box<Type>),
    Object(Box<ObjectType>),
}

impl From<ObjectType> for Type {
    fn from(object_type: ObjectType) -> Self {
        Type::Object(Box::new(object_type))
    }
}

impl Type {
    pub fn _as_struct(&self) -> Option<String> {
        match self {
            Type::Struct(name) => Some(name.clone()),
            _ => None,
        }
    }
    pub fn _is_struct(&self) -> bool {
        match self {
            Type::Struct(_) => true,
            _ => false,
        }
    }
    pub fn is_array(&self) -> bool {
        match self {
            Type::Array(_) => true,
            _ => false,
        }
    }
    pub fn get_array_type(&self) -> Option<Type> {
        match self {
            Type::Array(type_) => Some(*type_.clone()),
            _ => None,
        }
    }
    pub fn as_fn(&self) -> Option<Fn> {
        match self {
            Type::Fn(fn_) => Some(fn_.clone()),
            _ => None,
        }
    }
    pub fn is_fn(&self) -> bool {
        self.as_fn().is_some()
    }

    pub fn is_both_fn(&self, other: &Type) -> bool {
        self.is_fn() && other.is_fn()
    }
}

impl Display for Type {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let type_ = match self {
            Type::Fn(fn_) => format!("Fn({:?})", fn_.return_type),
            Type::Nil => "Nil".to_string(),
            Type::String => "String".to_string(),
            Type::Bool => "Bool".to_string(),
            Type::Int => "Int".to_string(),
            Type::Struct(name) => format!("Struct({:?})", name),
            Type::UnInitialized => "UnInitialized".to_string(),
            Type::Array(type_) => format!("Array({:?})", type_),
            Type::Object(type_) => format!("{:?}", type_),
            Type::Instance(name) => format!("Instance({:?})", name),
        };
        write!(f, "{type_}")
    }
}
