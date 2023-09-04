use std::fmt::{self, Display, Formatter};
use std::ops::Range;
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
    Literal(ExprLiteral),
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
}

impl Var {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Type {
    String,
    Number,
    Bool,
    Struct(Vec<Var>),
    None,
}
