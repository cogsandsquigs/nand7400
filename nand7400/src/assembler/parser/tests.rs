#![cfg(test)]

use super::*;
use std::collections::HashMap;

#[macro_use]
mod macros {
    /// Tests a list of tokens against the lexer's output.
    macro_rules! parses_as {
        ($parse_func:ident$(::<$($x:ty),+ $(,)?>)?, $input:expr, $ast:expr $(,)?) => {
            #[allow(unused_mut)]
            let mut parser = crate::assembler::parser::Parser::new($input).unwrap();

            let parsed_ast = parser.$parse_func$(::<$($x),+>)?().unwrap();

            assert_eq!(parsed_ast, $ast);
        };
    }
}

/// Test EOF parsing, and make sure that EOFs return the AST unchanged.
#[test]
fn parse_eof() {
    parses_as!(parse, "", Ast::empty());
}

/// Test the parsing of a label.
#[test]
fn parse_label() {
    parses_as!(
        parse,
        "label:",
        Ast {
            instructions: vec![Instruction {
                kind: InstructionKind::Label("label".to_string()),
                span: Position::new(0, 5),
            }],
            symbols: HashMap::from([("label".to_string(), 0)]),
        },
    );

    parses_as!(
        parse,
        "asdf123: \n   \n \n ",
        Ast {
            instructions: vec![Instruction {
                kind: InstructionKind::Label("asdf123".to_string()),
                span: Position::new(0, 7),
            }],
            symbols: HashMap::from([("asdf123".to_string(), 0)]),
        },
    );
}

/// Test the parsing of a number, both indirect, direct, positive, and negative.
#[test]
fn parse_number_prefixes() {
    parses_as!(
        parse_numeric_argument::<u8, i8>,
        "123",
        Argument {
            kind: ArgumentKind::ImmediateNumber(123),
            span: Position::new(0, 3),
        },
    );

    parses_as!(
        parse_numeric_argument::<u8, i8>,
        "+123",
        Argument {
            kind: ArgumentKind::ImmediateNumber(123),
            span: Position::new(0, 4),
        },
    );

    parses_as!(
        parse_numeric_argument::<u8, i8>,
        "-123",
        Argument {
            kind: ArgumentKind::ImmediateNumber(-123_i8 as u8),
            span: Position::new(0, 4),
        }
    );

    parses_as!(
        parse_numeric_argument::<u8, i8>,
        "#123",
        Argument {
            kind: ArgumentKind::IndirectNumber(123),
            span: Position::new(0, 4),
        },
    );

    parses_as!(
        parse_numeric_argument::<u8, i8>,
        "#+123",
        Argument {
            kind: ArgumentKind::IndirectNumber(123),
            span: Position::new(0, 5),
        },
    );

    parses_as!(
        parse_numeric_argument::<u8, i8>,
        "#-123",
        Argument {
            kind: ArgumentKind::IndirectNumber(-123_i8 as u8),
            span: Position::new(0, 5),
        }
    );
}

/// Test the parsing of numbers with different bases.
#[test]
fn parse_number_bases() {
    parses_as!(
        parse_numeric_argument::<u8, i8>,
        "0b101",
        Argument {
            kind: ArgumentKind::ImmediateNumber(0b101),
            span: Position::new(0, 5),
        },
    );

    parses_as!(
        parse_numeric_argument::<u8, i8>,
        "0o123",
        Argument {
            kind: ArgumentKind::ImmediateNumber(0o123),
            span: Position::new(0, 5),
        },
    );

    parses_as!(
        parse_numeric_argument::<u8, i8>,
        "0xFE",
        Argument {
            kind: ArgumentKind::ImmediateNumber(0xFE),
            span: Position::new(0, 4),
        },
    );
}
