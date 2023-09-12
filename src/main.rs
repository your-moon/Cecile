use lalrpop_util::lalrpop_mod;
use std::io::Write;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

use logos::{Logos, SpannedIter};

use crate::lexer::Token;
mod ast;
mod lexer;

lalrpop_mod!(pub grammar);

// write test

#[cfg(test)]

mod tests {
    use super::*;
    #[test]
    fn variable_declaration() {
        let input = r#"let variable_1: String = "Hello";"#;
        let mut lexer = lexer::Lexer::new(input).map(|token| match token {
            Ok((l, token, r)) => {
                println!("{:?}", token);
                return (l, token, r);
            }
            Err(_) => todo!("Error handling"),
        });
        let parser = grammar::ProgramParser::new();
        let program = parser.parse(lexer).unwrap();
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
        let input = r#"let variable_2: Int = 123;"#;
        let mut lexer = lexer::Lexer::new(input).map(|token| match token {
            Ok((l, token, r)) => {
                return (l, token, r);
            }
            Err(_) => todo!("Error handling"),
        });
        let parser = grammar::ProgramParser::new();
        let program = parser.parse(lexer).unwrap();
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
        let mut lexer = lexer::Lexer::new(input).map(|token| match token {
            Ok((l, token, r)) => {
                return (l, token, r);
            }
            Err(_) => todo!("Error handling"),
        });
        let parser = grammar::ProgramParser::new();
        let program = parser.parse(lexer).unwrap();
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
        let mut lexer = lexer::Lexer::new(input).map(|token| match token {
            Ok((l, token, r)) => {
                return (l, token, r);
            }
            Err(_) => todo!("Error handling"),
        });
        let parser = grammar::ProgramParser::new();
        let program = parser.parse(lexer).unwrap();
        for (statement, _range) in &program.statements {
            assert_eq!(
                statement,
                &ast::Statement::Return(ast::StatementReturn {
                    value: Some((
                        ast::Expression::Var(ast::ExprVar {
                            var: ast::Var {
                                name: "something".to_string(),
                                type_: None
                            }
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
        let mut lexer = lexer::Lexer::new(input).map(|token| match token {
            Ok((l, token, r)) => {
                return (l, token, r);
            }
            Err(_) => todo!("Error handling"),
        });
        let parser = grammar::ProgramParser::new();
        let program = parser.parse(lexer).unwrap();
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
        let mut lexer = lexer::Lexer::new(input).map(|token| match token {
            Ok((l, token, r)) => {
                return (l, token, r);
            }
            Err(_) => todo!("Error handling"),
        });
        let parser = grammar::ProgramParser::new();
        let program = parser.parse(lexer).unwrap();
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
        let mut lexer = lexer::Lexer::new(input).map(|token| match token {
            Ok((l, token, r)) => {
                return (l, token, r);
            }
            Err(_) => todo!("Error handling"),
        });
        let parser = grammar::ProgramParser::new();
        let program = parser.parse(lexer).unwrap();
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
        let mut lexer = lexer::Lexer::new(input).map(|token| match token {
            Ok((l, token, r)) => {
                return (l, token, r);
            }
            Err(_) => todo!("Error handling"),
        });
        let parser = grammar::ProgramParser::new();
        let program = parser.parse(lexer).unwrap();
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

fn main() {
    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Magenta)));
    writeln!(&mut stdout, "magenta text!");
}
