use std::default;
use std::fmt::{self, Display, Formatter};
use std::ops::Range;

use hashbrown::HashMap;
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
pub struct StatementFun {
    pub name: String,
    pub params: HashMap<String, Option<Type>>,
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

impl StatementVar {
    pub fn new(name: String, type_: Option<Type>, value: Option<ExprS>) -> Self {
        let var = Var { name, type_ };
        Self { var, value }
    }
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
    Call(Box<ExprCall>),
    Literal(ExprLiteral),
    Var(ExprVar),
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

#[derive(Clone, Debug, PartialEq, Default)]
pub enum Type {
    Fn,
    Nil,
    String,
    Bool,
    Int,
    Object,
    #[default]
    UnInitialized,
}

impl Display for Type {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let type_ = match self {
            Type::Fn => "fn",
            Type::Nil => "nil",
            Type::String => "String",
            Type::Bool => "Bool",
            Type::Int => "Int",
            Type::Object => "Object",
            Type::UnInitialized => "Uninitialized",
        };
        write!(f, "{type_}")
    }
}
