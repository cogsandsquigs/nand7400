#![cfg(test)]

use super::*;
use miette::SourceSpan;

/// Test the parsing of an argument.
#[test]
fn test_argument() {
    assert_eq!(
        argument(Span::new("0x1A 123")).unwrap().1,
        Argument::new(
            ArgumentKind::Literal(0x1A),
            SourceSpan::new(0.into(), 4.into())
        )
    );

    assert_eq!(
        argument(Span::new("0b0010_1010_234567890abcdefg 123"))
            .unwrap()
            .1,
        Argument::new(
            ArgumentKind::Literal(0b00101010),
            SourceSpan::new(0.into(), 12.into())
        )
    );

    assert_eq!(
        argument(Span::new("0o067_890abcdefg 123")).unwrap().1,
        Argument::new(
            ArgumentKind::Literal(0o067),
            SourceSpan::new(0.into(), 6.into())
        )
    );

    assert_eq!(
        argument(Span::new("123_abc")).unwrap().1,
        Argument::new(
            ArgumentKind::Literal(123),
            SourceSpan::new(0.into(), 4.into())
        )
    );

    assert_eq!(
        argument(Span::new("foo_bar_123 asdfawefi3")).unwrap().1,
        Argument::new(
            ArgumentKind::Label("foo_bar_123".to_string()),
            SourceSpan::new(0.into(), 11.into())
        )
    );
}
