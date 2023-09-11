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
            println!("{:?}", statement);
        }
    }
}

fn main() {
    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Magenta)));
    writeln!(&mut stdout, "magenta text!");
}
