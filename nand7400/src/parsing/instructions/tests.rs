#![cfg(test)]

use super::*;
use crate::{config::Opcode, parsing::State};
use miette::SourceSpan;

/// Test the parsing of an argument.
#[test]
fn test_parse_argument() {
    assert_eq!(
        argument(Span::new_extra("0x1A 123", State::new()))
            .unwrap()
            .1,
        Argument::new(
            ArgumentKind::Literal(0x1A),
            SourceSpan::new(0.into(), 4.into())
        )
    );

    assert_eq!(
        argument(Span::new_extra(
            "0b0010_1010_234567890abcdefg 123",
            State::new()
        ))
        .unwrap()
        .1,
        Argument::new(
            ArgumentKind::Literal(0b00101010),
            SourceSpan::new(0.into(), 12.into())
        )
    );

    assert_eq!(
        argument(Span::new_extra("0o067_890abcdefg 123", State::new()))
            .unwrap()
            .1,
        Argument::new(
            ArgumentKind::Literal(0o067),
            SourceSpan::new(0.into(), 6.into())
        )
    );

    assert_eq!(
        argument(Span::new_extra("123_abc", State::new()))
            .unwrap()
            .1,
        Argument::new(
            ArgumentKind::Literal(123),
            SourceSpan::new(0.into(), 4.into())
        )
    );

    assert_eq!(
        argument(Span::new_extra("foo_bar_123 asdfawefi3", State::new()))
            .unwrap()
            .1,
        Argument::new(
            ArgumentKind::Label("foo_bar_123".to_string()),
            SourceSpan::new(0.into(), 11.into())
        )
    );
}

/// Test the parsing of a single instruction.
#[test]
fn test_parse_instruction() {
    let opcodes = vec![Opcode {
        name: "foo".to_string(),
        binary: 0xCA,
        num_args: 0,
    }];

    assert_eq!(
        instruction(&opcodes,)(Span::new_extra("foo", State::new()),)
            .unwrap()
            .1
            .unwrap()
            .opcode
            .name,
        "foo"
    );

    assert_eq!(
        instruction(&opcodes,)(Span::new_extra("foo 0xCA", State::new()),)
            .unwrap()
            .1
            .unwrap()
            .arguments,
        vec![Argument::new(
            ArgumentKind::Literal(0x1A),
            SourceSpan::new(4.into(), 4.into())
        )]
    );
}

/// Test the parsing of labels.
#[test]
fn test_parse_label() {
    assert_eq!(
        label(0)(Span::new_extra("foo:", State::new())).unwrap().1,
        Label {
            name: "foo".to_string(),
            value: 1,
            span: SourceSpan::new(0.into(), 3.into())
        }
    );
}
