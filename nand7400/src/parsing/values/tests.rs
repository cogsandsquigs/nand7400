#![cfg(test)]

use super::*;
use crate::parsing::State;

/// Test parsing a decimal value, seeing if we stop at the right place (before the extra characters).
#[test]
fn test_decimal() {
    assert_eq!(
        decimal(Span::new_extra("012_6_abcdefg", State::new()))
            .unwrap()
            .1,
        126
    );

    assert_eq!(
        decimal(Span::new_extra("-127", State::new())).unwrap().1,
        -127
    );
}

/// Test parsing a hexadecimal value, seeing if we stop at the right place (before the extra characters).
#[test]
fn test_hexadecimal() {
    assert_eq!(
        hexadecimal(Span::new_extra("0x0F_ghijk", State::new()))
            .unwrap()
            .1,
        0x0F
    );

    assert_eq!(
        hexadecimal(Span::new_extra("-0x7F", State::new()))
            .unwrap()
            .1,
        -0x7F
    );
}

/// Test parsing an octal value, seeing if we stop at the right place (before the extra characters).
#[test]
fn test_octal() {
    assert_eq!(
        octal(Span::new_extra("0o067_890abcdefg", State::new()))
            .unwrap()
            .1,
        0o067
    );

    assert_eq!(
        octal(Span::new_extra("-0o177", State::new())).unwrap().1,
        -0o177
    );
}

/// Test parsing a binary value, seeing if we stop at the right place (before the extra characters).
#[test]
fn test_binary() {
    assert_eq!(
        binary(Span::new_extra(
            "0b0010_1010_234567890abcdefg",
            State::new()
        ))
        .unwrap()
        .1,
        0b00101010
    );

    assert_eq!(
        binary(Span::new_extra("-0b0111_1111", State::new()))
            .unwrap()
            .1,
        -0b0111_1111
    );
}

/// Test parsing an identifier, seeing if we stop at the right place (before the extra characters).
#[test]
fn test_identifier() {
    assert_eq!(
        identifier(Span::new_extra("foo_bar_123 asdfawefi3", State::new()))
            .unwrap()
            .1,
        "foo_bar_123"
    );
}
