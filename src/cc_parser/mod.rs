pub mod ast;
use lalrpop_util::{lalrpop_mod, ParseError};
lalrpop_mod!(pub grammar, "/src/cc_parser/grammar.rs");

// write test
use crate::{
    cc_lexer::Lexer,
    cc_parser::ast::{Fn, Type},
    vm::{
        compiler_globals::CompilerGlobals,
        error::{Error, ErrorS, SyntaxError},
    },
};

use self::ast::Program;

pub fn is_complete(source: &str) -> bool {
    let lexer = Lexer::new(source);
    let parser = grammar::ProgramParser::new();
    let mut errors = Vec::new();
    if let Err(e) = parser.parse(&mut errors, lexer) {
        errors.push(e);
    };
    !errors
        .iter()
        .any(|e| matches!(e, ParseError::UnrecognizedEof { .. }))
}

pub fn parse(
    source: &str,
    offset: usize,
    ast_debug: bool,
    globals: &mut CompilerGlobals,
) -> Result<Program, Vec<ErrorS>> {
    let lexer = Lexer::new(source).map(|token| match token {
        Ok((l, token, r)) => Ok((l + offset, token, r + offset)),
        Err((e, span)) => Err((e, span.start + offset..span.end + offset)),
    });
    let parser = grammar::ProgramParser::new();

    let mut errors = Vec::new();
    let mut parse_errors = Vec::new();
    let program = match parser.parse(&mut parse_errors, lexer) {
        Ok(mut program) => program.sort_structs(),
        Err(err) => {
            parse_errors.push(err);
            Program::default()
        }
    };
    for (statement, _range) in &program.statements {
        if statement.is_fun() {
            let fun = statement.as_fun().unwrap();
            globals.insert(
                fun.name.clone(),
                Type::Fn(Fn {
                    return_type: Box::new(fun.return_type.clone()),
                }),
            );
        }
        if ast_debug {
            println!("");
            println!("{:?}", statement);
        }
    }
    errors.extend(parse_errors.into_iter().map(|err| match err {
        ParseError::ExtraToken {
            token: (start, _, end),
        } => (
            Error::SyntaxError(SyntaxError::ExtraToken {
                token: source[start..end].to_string(),
            }),
            start..end,
        ),
        ParseError::InvalidToken { location } => (
            Error::SyntaxError(SyntaxError::InvalidToken),
            location..location,
        ),
        ParseError::UnrecognizedEof { location, expected } => (
            Error::SyntaxError(SyntaxError::UnrecognizedEOF { expected }),
            location..location,
        ),
        ParseError::UnrecognizedToken {
            token: (start, _, end),
            expected,
        } => (
            Error::SyntaxError(SyntaxError::UnrecognizedToken {
                token: source[start..end].to_string(),
                expected,
            }),
            start..end,
        ),
        ParseError::User { error } => error,
    }));

    if errors.is_empty() {
        Ok(program)
    } else {
        Err(errors)
    }
}

#[cfg(test)]

mod tests {

    use crate::cc_lexer::Lexer;

    use crate::cc_parser::ast::{ExprInfix, ExprLiteral, Expression};

    use super::*;

    #[test]
    fn if_statement_declaration_2() {
        let input = r#"if(true) { print 1; }"#;
        let lexer = Lexer::new(input).map(|token| match token {
            Ok((l, token, r)) => (l, token, r),
            Err(_) => todo!("Error handling"),
        });
        let parser = grammar::ProgramParser::new();
        let mut parse_errors = Vec::new();
        let program = parser.parse(&mut parse_errors, lexer).unwrap();
        for (statement, _range) in &program.statements {
            assert_eq!(
                statement,
                &ast::Statement::If(Box::new(ast::StatementIf {
                    cond: (ast::Expression::Literal(ast::ExprLiteral::Bool(true)), 3..7),
                    then_branch: (
                        ast::Statement::Block(ast::StatementBlock {
                            statements: vec![(
                                ast::Statement::Print(ast::StatementPrint {
                                    value: (
                                        ast::Expression::Literal(ast::ExprLiteral::Number(1.0)),
                                        17..18
                                    )
                                }),
                                11..19
                            )]
                        }),
                        9..21
                    ),
                    else_branch: None
                }))
            );
        }
    }

    #[test]
    fn if_statement_declaration() {
        let input = r#"if(true) {}"#;
        let lexer = Lexer::new(input).map(|token| match token {
            Ok((l, token, r)) => {
                println!("{:?}", token);
                return (l, token, r);
            }
            Err(_) => todo!("Error handling"),
        });
        let parser = grammar::ProgramParser::new();
        let mut parse_errors = Vec::new();
        let program = parser.parse(&mut parse_errors, lexer).unwrap();
        for (statement, _range) in &program.statements {
            assert_eq!(
                statement,
                &ast::Statement::If(Box::new(ast::StatementIf {
                    cond: (ast::Expression::Literal(ast::ExprLiteral::Bool(true)), 3..7),
                    then_branch: (
                        ast::Statement::Block(ast::StatementBlock { statements: vec![] }),
                        9..11
                    ),
                    else_branch: None
                }))
            );
        }
    }

    #[test]
    fn while_statement_declaration() {
        let input = r#"while(true) {}"#;
        let lexer = Lexer::new(input).map(|token| match token {
            Ok((l, token, r)) => {
                println!("{:?}", token);
                return (l, token, r);
            }
            Err(_) => todo!("Error handling"),
        });
        let parser = grammar::ProgramParser::new();
        let mut parse_errors = Vec::new();
        let program = parser.parse(&mut parse_errors, lexer).unwrap();
        for (statement, _range) in &program.statements {
            assert_eq!(
                statement,
                &ast::Statement::While(Box::new(ast::StatementWhile {
                    cond: (
                        ast::Expression::Literal(ast::ExprLiteral::Bool(true)),
                        6..10
                    ),
                    body: (
                        ast::Statement::Block(ast::StatementBlock { statements: vec![] }),
                        12..14
                    )
                }))
            );
        }
    }

    #[test]
    fn for_statement_declaration() {
        let input = r#"for(let i = 0; i<10; i=i+1) {}"#;
        let lexer = Lexer::new(input).map(|token| match token {
            Ok((l, token, r)) => {
                println!("{:?}", token);
                return (l, token, r);
            }
            Err(_) => todo!("Error handling"),
        });
        let parser = grammar::ProgramParser::new();
        let mut parse_errors = Vec::new();
        let program = parser.parse(&mut parse_errors, lexer).unwrap();
        for (statement, _range) in &program.statements {
            println!("STATEMENT {:?}", statement);
            assert_eq!(
                statement,
                &ast::Statement::For(Box::new(ast::StatementFor {
                    init: Some((
                        ast::Statement::Var(ast::StatementVar {
                            var: ast::Var {
                                name: "i".to_string(),
                                type_: None
                            },
                            value: Some((
                                ast::Expression::Literal(ast::ExprLiteral::Number(0.0)),
                                12..13
                            )),
                        }),
                        4..14
                    )),
                    cond: Some((
                        ast::Expression::Infix(Box::new(ast::ExprInfix {
                            lhs: (
                                ast::Expression::Var(ast::ExprVar {
                                    name: "i".to_string(),
                                }),
                                15..16
                            ),
                            op: ast::OpInfix::Less,
                            rhs: (
                                ast::Expression::Literal(ast::ExprLiteral::Number(10.0)),
                                17..19
                            )
                        })),
                        15..19
                    )),
                    update: Some((
                        ast::Expression::Assign(Box::new(ast::ExprAssign {
                            lhs: ast::Var {
                                name: "i".to_string(),
                                type_: None
                            },

                            rhs: (
                                Expression::Infix(Box::new(ExprInfix {
                                    lhs: (
                                        Expression::Var(ast::ExprVar {
                                            name: "i".to_string(),
                                        }),
                                        23..24
                                    ),
                                    op: ast::OpInfix::Add,
                                    rhs: (Expression::Literal(ExprLiteral::Number(1.0)), 25..26)
                                })),
                                23..26
                            )
                        })),
                        21..26
                    )),

                    body: (
                        ast::Statement::Block(ast::StatementBlock { statements: vec![] }),
                        28..30
                    )
                }))
            );
        }
    }

    #[test]
    fn variable_declaration() {
        let input = r#"let variable_1: String = "Hello";"#;
        let lexer = Lexer::new(input).map(|token| match token {
            Ok((l, token, r)) => {
                println!("{:?}", token);
                return (l, token, r);
            }
            Err(_) => todo!("Error handling"),
        });
        let parser = grammar::ProgramParser::new();
        let mut parse_errors = Vec::new();
        let program = parser.parse(&mut parse_errors, lexer).unwrap();
        for (statement, _range) in &program.statements {
            assert_eq!(
                statement,
                &ast::Statement::Var(ast::StatementVar {
                    var: ast::Var {
                        name: "variable_1".to_string(),
                        type_: Some(ast::Type::String),
                    },
                    value: Some((
                        ast::Expression::Literal(ast::ExprLiteral::String("Hello".to_string())),
                        25..32
                    )),
                })
            );
        }
    }

    #[test]
    fn variable_declaration_int() {
        let input = r#"let variable_2: int = 123;"#;
        let lexer = Lexer::new(input).map(|token| match token {
            Ok((l, token, r)) => {
                return (l, token, r);
            }
            Err(_) => todo!("Error handling"),
        });
        let parser = grammar::ProgramParser::new();
        let mut parse_errors = Vec::new();
        let program = parser.parse(&mut parse_errors, lexer).unwrap();
        for (statement, _range) in &program.statements {
            assert_eq!(
                statement,
                &ast::Statement::Var(ast::StatementVar {
                    var: ast::Var {
                        name: "variable_2".to_string(),
                        type_: Some(ast::Type::Int),
                    },
                    value: Some((
                        ast::Expression::Literal(ast::ExprLiteral::Number(123.0)),
                        22..25
                    )),
                })
            );
        }
    }

    #[test]
    fn block_statement() {
        let input = r#"{}"#;
        let lexer = Lexer::new(input).map(|token| match token {
            Ok((l, token, r)) => {
                return (l, token, r);
            }
            Err(_) => todo!("Error handling"),
        });
        let parser = grammar::ProgramParser::new();
        let mut parse_errors = Vec::new();
        let program = parser.parse(&mut parse_errors, lexer).unwrap();
        for (statement, _range) in &program.statements {
            assert_eq!(
                statement,
                &ast::Statement::Block(ast::StatementBlock { statements: vec![] })
            );
        }
    }

    #[test]
    fn return_statement() {
        let input = r#"return something;"#;
        let lexer = Lexer::new(input).map(|token| match token {
            Ok((l, token, r)) => {
                return (l, token, r);
            }
            Err(_) => todo!("Error handling"),
        });
        let parser = grammar::ProgramParser::new();
        let mut parse_errors = Vec::new();
        let program = parser.parse(&mut parse_errors, lexer).unwrap();
        for (statement, _range) in &program.statements {
            assert_eq!(
                statement,
                &ast::Statement::Return(ast::StatementReturn {
                    value: Some((
                        ast::Expression::Var(ast::ExprVar {
                            name: "something".to_string(),
                        }),
                        7..16
                    ))
                })
            );
        }
    }

    #[test]
    fn print_statement() {
        let input = r#"print 1;"#;
        let lexer = Lexer::new(input).map(|token| match token {
            Ok((l, token, r)) => {
                return (l, token, r);
            }
            Err(_) => todo!("Error handling"),
        });
        let parser = grammar::ProgramParser::new();
        let mut parse_errors = Vec::new();
        let program = parser.parse(&mut parse_errors, lexer).unwrap();
        for (statement, _range) in &program.statements {
            assert_eq!(
                statement,
                &ast::Statement::Print(ast::StatementPrint {
                    value: (
                        ast::Expression::Literal(ast::ExprLiteral::Number(1.0)),
                        6..7
                    )
                })
            );
        }
    }

    #[test]
    fn div_expression() {
        let input = r#"1 / 2;"#;
        let lexer = Lexer::new(input).map(|token| match token {
            Ok((l, token, r)) => {
                return (l, token, r);
            }
            Err(_) => todo!("Error handling"),
        });
        let parser = grammar::ProgramParser::new();
        let mut parse_errors = Vec::new();
        let program = parser.parse(&mut parse_errors, lexer).unwrap();
        for (statement, _range) in &program.statements {
            assert_eq!(
                statement,
                &ast::Statement::Expression(ast::StatementExpr {
                    expr: (
                        ast::Expression::Infix(Box::new(ast::ExprInfix {
                            lhs: (
                                ast::Expression::Literal(ast::ExprLiteral::Number(1.0)),
                                0..1
                            ),
                            op: ast::OpInfix::Div,
                            rhs: (
                                ast::Expression::Literal(ast::ExprLiteral::Number(2.0)),
                                4..5
                            )
                        })),
                        0..5
                    )
                })
            );
        }
    }

    #[test]
    fn mul_expression() {
        let input = r#"1 * 2;"#;
        let lexer = Lexer::new(input).map(|token| match token {
            Ok((l, token, r)) => {
                return (l, token, r);
            }
            Err(_) => todo!("Error handling"),
        });
        let parser = grammar::ProgramParser::new();
        let mut parse_errors = Vec::new();
        let program = parser.parse(&mut parse_errors, lexer).unwrap();
        for (statement, _range) in &program.statements {
            assert_eq!(
                statement,
                &ast::Statement::Expression(ast::StatementExpr {
                    expr: (
                        ast::Expression::Infix(Box::new(ast::ExprInfix {
                            lhs: (
                                ast::Expression::Literal(ast::ExprLiteral::Number(1.0)),
                                0..1
                            ),
                            op: ast::OpInfix::Mul,
                            rhs: (
                                ast::Expression::Literal(ast::ExprLiteral::Number(2.0)),
                                4..5
                            )
                        })),
                        0..5
                    )
                })
            );
        }
    }

    #[test]
    fn negate_expression() {
        let input = r#"-1;"#;
        let lexer = Lexer::new(input).map(|token| match token {
            Ok((l, token, r)) => {
                return (l, token, r);
            }
            Err(_) => todo!("Error handling"),
        });
        let parser = grammar::ProgramParser::new();
        let mut parse_errors = Vec::new();
        let program = parser.parse(&mut parse_errors, lexer).unwrap();
        for (statement, _range) in &program.statements {
            assert_eq!(
                statement,
                &ast::Statement::Expression(ast::StatementExpr {
                    expr: (
                        ast::Expression::Prefix(Box::new(ast::ExprPrefix {
                            op: ast::OpPrefix::Negate,
                            rt: (
                                ast::Expression::Literal(ast::ExprLiteral::Number(1.0)),
                                1..2
                            )
                        })),
                        0..2
                    )
                })
            );
        }
    }
}
