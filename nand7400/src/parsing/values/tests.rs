#![cfg(test)]

use super::*;

/// Test parsing a decimal value, seeing if we stop at the right place (before the extra characters).
#[test]
fn test_decimal() {
    assert_eq!(decimal(Span::new("012_9_abcdefg")).unwrap().1, 129);
}

/// Test parsing a hexadecimal value, seeing if we stop at the right place (before the extra characters).
#[test]
fn test_hexadecimal() {
    assert_eq!(hexadecimal(Span::new("0xF0_ghijk")).unwrap().1, 0xF0);
}

/// Test parsing an octal value, seeing if we stop at the right place (before the extra characters).
#[test]
fn test_octal() {
    assert_eq!(octal(Span::new("0o067_890abcdefg")).unwrap().1, 0o067);
}

/// Test parsing a binary value, seeing if we stop at the right place (before the extra characters).
#[test]
fn test_binary() {
    assert_eq!(
        binary(Span::new("0b1010_1010_234567890abcdefg")).unwrap().1,
        0b10101010
    );
}

/// Test parsing an identifier, seeing if we stop at the right place (before the extra characters).
#[test]
fn test_identifier() {
    assert_eq!(
        identifier(Span::new("foo_bar_123 asdfawefi3")).unwrap().1,
        "foo_bar_123"
    );
}
