#![cfg(test)]

use super::*;
use std::collections::HashMap;

/// Tests a list of tokens against the lexer's output.
fn parses_as(input: &str, ast: Ast) {
    let parser = Parser::new(input).unwrap();

    let parsed_ast = parser.parse().unwrap();

    assert_eq!(parsed_ast, ast);
}

/// Test EOF parsing, and make sure that EOFs return the AST unchanged.
#[test]
fn parse_eof() {
    parses_as("", Ast::empty());
}

/// Test the parsing of a label.
#[test]
fn parse_label() {
    parses_as(
        "label:",
        Ast {
            instructions: vec![Instruction {
                kind: InstructionKind::Label("label".to_string()),
                span: Position::new(0, 5),
            }],
            symbols: HashMap::from([("label".to_string(), 0)]),
        },
    );

    parses_as(
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
fn parse_number() {
    let mut parser = Parser::new("123").unwrap();

    let parsed = parser.parse_numeric_argument::<u8, i8>().unwrap();

    assert_eq!(
        parsed,
        Argument {
            kind: ArgumentKind::ImmediateNumber(123),
            span: Position::new(0, 3),
        }
    );

    let mut parser = Parser::new("+123").unwrap();

    let parsed = parser.parse_numeric_argument::<u8, i8>().unwrap();

    assert_eq!(
        parsed,
        Argument {
            kind: ArgumentKind::ImmediateNumber(123),
            span: Position::new(0, 4),
        }
    );

    let mut parser = Parser::new("-123").unwrap();

    let parsed = parser.parse_numeric_argument::<u8, i8>().unwrap();

    assert_eq!(
        parsed,
        Argument {
            kind: ArgumentKind::ImmediateNumber(-123_i8 as u8),
            span: Position::new(0, 4),
        }
    );

    let mut parser = Parser::new("#123").unwrap();

    let parsed = parser.parse_numeric_argument::<u8, i8>().unwrap();

    assert_eq!(
        parsed,
        Argument {
            kind: ArgumentKind::IndirectNumber(123),
            span: Position::new(0, 4),
        }
    );

    let mut parser = Parser::new("#+123").unwrap();

    let parsed = parser.parse_numeric_argument::<u8, i8>().unwrap();

    assert_eq!(
        parsed,
        Argument {
            kind: ArgumentKind::IndirectNumber(123),
            span: Position::new(0, 5),
        }
    );

    let mut parser = Parser::new("#-123").unwrap();

    let parsed = parser.parse_numeric_argument::<u8, i8>().unwrap();

    assert_eq!(
        parsed,
        Argument {
            kind: ArgumentKind::IndirectNumber(-123_i8 as u8),
            span: Position::new(0, 5),
        }
    );
}

/// Test the parsing of numbers with different bases.
#[test]
fn parse_number_base() {
    let mut parser = Parser::new("0b1010").unwrap();

    let parsed = parser.parse_numeric_argument::<u8, i8>().unwrap();

    assert_eq!(
        parsed,
        Argument {
            kind: ArgumentKind::ImmediateNumber(0b1010),
            span: Position::new(0, 6),
        }
    );

    let mut parser = Parser::new("0o123").unwrap();

    let parsed = parser.parse_numeric_argument::<u8, i8>().unwrap();

    assert_eq!(
        parsed,
        Argument {
            kind: ArgumentKind::ImmediateNumber(0o123),
            span: Position::new(0, 5),
        }
    );

    let mut parser = Parser::new("0xFE").unwrap();

    let parsed = parser.parse_numeric_argument::<u8, i8>().unwrap();

    assert_eq!(
        parsed,
        Argument {
            kind: ArgumentKind::ImmediateNumber(0xFE),
            span: Position::new(0, 4),
        }
    );
}
