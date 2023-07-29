#![cfg(test)]

use super::*;

/// Test EOF parsing, and make sure that EOFs return the AST unchanged.
#[test]
fn parse_eof() {
    let parser = Parser::new("").unwrap();
    let ast = parser.parse().unwrap();

    assert_eq!(ast, Ast::empty());
}
