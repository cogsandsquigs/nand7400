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
