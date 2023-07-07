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
            "0b00101010 234567890abcdefg 123",
            State::new()
        ))
        .unwrap()
        .1,
        Argument::new(
            ArgumentKind::Literal(0b00101010),
            SourceSpan::new(0.into(), 10.into())
        )
    );

    assert_eq!(
        argument(Span::new_extra("0o067 890abcdefg 123", State::new()))
            .unwrap()
            .1,
        Argument::new(
            ArgumentKind::Literal(0o067),
            SourceSpan::new(0.into(), 5.into())
        )
    );

    assert_eq!(
        argument(Span::new_extra("123 abc", State::new()))
            .unwrap()
            .1,
        Argument::new(
            ArgumentKind::Literal(123),
            SourceSpan::new(0.into(), 3.into())
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

/// Test the parsing of a single instruction, as well as a variety of errors (too many args).
#[test]
fn test_parse_instruction_too_many() {
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
        instruction(&opcodes,)(Span::new_extra("foo 0x1A 0b0 0o77 12", State::new()),)
            .unwrap()
            .0
            .extra
            .errors
            .borrow()
            .as_ref(),
        vec![ParsingError::TooManyArgs {
            opcode: "foo".into(),
            expected: 0,
            got: 4,
            span: SourceSpan::new(4.into(), 16.into())
        }]
    );
}

/// Test the parsing of a single instruction, as well as a variety of errors (too few args).
#[test]
fn test_parse_instruction_too_few() {
    let opcodes = vec![Opcode {
        name: "bar".to_string(),
        binary: 0x12,
        num_args: 2,
    }];

    assert_eq!(
        instruction(&opcodes,)(Span::new_extra("bar", State::new()),)
            .unwrap()
            .0
            .extra
            .errors
            .borrow()
            .as_ref(),
        vec![ParsingError::TooFewArgs {
            opcode: "bar".into(),
            expected: 2,
            got: 0,
            span: SourceSpan::new(3.into(), 3.into())
        },]
    );

    assert_eq!(
        instruction(&opcodes,)(Span::new_extra("bar 1", State::new()),)
            .unwrap()
            .0
            .extra
            .errors
            .borrow()
            .as_ref(),
        vec![ParsingError::TooFewArgs {
            opcode: "bar".into(),
            expected: 2,
            got: 1,
            span: SourceSpan::new(5.into(), 5.into())
        }]
    );

    assert_eq!(
        instruction(&opcodes,)(Span::new_extra("bar 0x1A 0b0", State::new()),)
            .unwrap()
            .0
            .extra
            .errors
            .borrow()
            .as_ref(),
        vec![]
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
