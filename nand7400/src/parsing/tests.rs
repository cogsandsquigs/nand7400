#![cfg(test)]

use super::{AssemblyParser, Rule};
use pest::{consumes_to, parses_to};

/// Test parsing a literal value.
#[test]
fn test_literal() {
    parses_to! {
        parser: AssemblyParser,
        input:  "012345689",
        rule:   Rule::Literal,
        tokens: [
            Literal(0, 9)
        ]
    };

    parses_to! {
        parser: AssemblyParser,
        input:  "0x0123456789ABCDEF",
        rule:   Rule::Literal,
        tokens: [
            Literal(0, 18)
        ]
    };

    parses_to! {
        parser: AssemblyParser,
        input:  "0X0123456789abcdef",
        rule:   Rule::Literal,
        tokens: [
            Literal(0, 18)
        ]
    };

    parses_to! {
        parser: AssemblyParser,
        input:  "0b101",
        rule:   Rule::Literal,
        tokens: [
            Literal(0, 5)
        ]
    };

    parses_to! {
        parser: AssemblyParser,
        input:  "0o01234567",
        rule:   Rule::Literal,
        tokens: [
            Literal(0, 10)
        ]
    };
}
