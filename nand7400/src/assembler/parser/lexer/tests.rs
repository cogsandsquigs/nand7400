#![cfg(test)]

use super::*;
use crate::assembler::position::Position;

/// Tests a list of tokens against the lexer's output.
fn lexes_as(input: &str, tests: Vec<Token>) {
    let mut lexer = Lexer::new(input);

    for (i, tt) in tests.iter().enumerate() {
        let token = lexer.next_token().unwrap();

        assert_eq!(
            token.kind, tt.kind,
            "tests[{}] failed - wrong token type.",
            i
        );

        assert_eq!(
            token.literal, tt.literal,
            "tests[{}] failed - wrong literal value.",
            i
        );

        assert_eq!(
            token.position, tt.position,
            "tests[{}] failed - wrong position.",
            i
        );
    }
}

/// Tests lexing until the EOF, and expects that it just returns an EOF even after "going past" the end of the file.
#[test]
fn lex_eof() {
    let input = "";

    let tests = vec![
        Token::new(TokenKind::Eof, Position::new(0, 0), "\0"),
        Token::new(TokenKind::Eof, Position::new(0, 0), "\0"),
    ];

    lexes_as(input, tests)
}

/// Tests the lexer's ability to handle positions correctly, including newlines with
/// `\n` and `\r\n` (both count as 1 newline, because windows :/).
#[test]
fn lex_positions() {
    let input = "abcde\nfghij\r\nklmno\r\n123\r 456\n789";

    let mut lexer = Lexer::new(input);

    let tests = vec![
        Position::new(0, 5),
        Position::new(5, 6),
        Position::new(6, 11),
        Position::new(11, 13),
        Position::new(13, 18),
        Position::new(18, 20),
        Position::new(20, 23),
        Position::new(23, 24),
        Position::new(25, 28),
    ];

    for (i, test) in tests.iter().enumerate() {
        let token = lexer.next_token();
        assert_eq!(&token.unwrap().position, test, "test[{}]: FAILED", i);
    }
}

/// Tests the lexer's ability to handle special characters.
#[test]
fn lex_special_characters() {
    let input = "+ : # -";

    let tests = vec![
        Token::new(TokenKind::Plus, Position::new(0, 1), "+"),
        Token::new(TokenKind::Colon, Position::new(2, 3), ":"),
        Token::new(TokenKind::Hash, Position::new(4, 5), "#"),
        Token::new(TokenKind::Minus, Position::new(6, 7), "-"),
    ];

    lexes_as(input, tests)
}

/// Tests the lexer's ability to tokenize different keywords.
#[test]
fn lex_keywords() {
    let input = ".byte .org";

    let tests = vec![
        Token::new(TokenKind::Keyword, Position::new(0, 5), ".byte"),
        Token::new(TokenKind::Keyword, Position::new(6, 10), ".org"),
    ];

    lexes_as(input, tests)
}

/// Tests the lexer's ability to tokenize identifiers.
#[test]
fn lex_identifiers() {
    let input = "abcde fghij kl_mno OIE ab12CE";

    let tests = vec![
        Token::new(TokenKind::Ident, Position::new(0, 5), "abcde"),
        Token::new(TokenKind::Ident, Position::new(6, 11), "fghij"),
        Token::new(TokenKind::Ident, Position::new(12, 18), "kl_mno"),
        Token::new(TokenKind::Ident, Position::new(19, 22), "OIE"),
        Token::new(TokenKind::Ident, Position::new(23, 29), "ab12CE"),
    ];

    lexes_as(input, tests)
}

/// Tests the lexer's ability to tokenize different numbers, including hex, binary, and octal. These are all byte values.
/// Note that the lexer does not support floating point numbers.
#[test]
fn lex_numeric_values() {
    let input = "0x3C 0b1010 0o156 0XFA 0B0101 0O123 1234 0x 0b 0o";

    let tests = vec![
        Token::new(TokenKind::Number, Position::new(0, 4), "0x3C"),
        Token::new(TokenKind::Number, Position::new(5, 11), "0b1010"),
        Token::new(TokenKind::Number, Position::new(12, 17), "0o156"),
        Token::new(TokenKind::Number, Position::new(18, 22), "0XFA"),
        Token::new(TokenKind::Number, Position::new(23, 29), "0B0101"),
        Token::new(TokenKind::Number, Position::new(30, 35), "0O123"),
        Token::new(TokenKind::Number, Position::new(36, 40), "1234"),
        Token::new(TokenKind::Number, Position::new(41, 43), "0x"),
        Token::new(TokenKind::Number, Position::new(44, 46), "0b"),
        Token::new(TokenKind::Number, Position::new(47, 49), "0o"),
    ];

    lexes_as(input, tests)
}

/// Test failing conditions for the lexer -- unknown characters and keywords.
#[test]
fn lex_failing_conditions() {
    let input = "% !";

    let tests = vec![
        ParsingError::UnknownCharacter {
            character: '%',
            span: Position::new(0, 1),
        },
        ParsingError::UnknownCharacter {
            character: '!',
            span: Position::new(2, 3),
        },
    ];

    let mut lexer = Lexer::new(input);

    for (i, test) in tests.into_iter().enumerate() {
        let token = lexer.next_token();
        assert_eq!(token.unwrap_err(), test, "test[{}]: FAILED", i);
    }
}
