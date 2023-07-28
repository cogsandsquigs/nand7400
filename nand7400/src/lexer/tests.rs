#![cfg(test)]

use super::*;
use crate::position::Position;

/// Tests a list of tokens against the lexer's output.
fn match_against(input: &str, tests: Vec<Token>) {
    let mut lexer = Lexer::new(input);

    for (i, tt) in tests.iter().enumerate() {
        let token = lexer.next_token();

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
        assert_eq!(&token.position, test, "test[{}]: FAILED", i);
    }
}

/// Tests the lexer's ability to handle special characters.
#[test]
fn lex_special_characters() {
    let input = "+ : # -";

    let tests = vec![
        Token::new(TokenType::Plus, Position::new(0, 1), "+"),
        Token::new(TokenType::Colon, Position::new(2, 3), ":"),
        Token::new(TokenType::Hash, Position::new(4, 5), "#"),
        Token::new(TokenType::Minus, Position::new(6, 7), "-"),
    ];

    match_against(input, tests)
}

/// Tests the lexer's ability to tokenize different keywords.
#[test]
fn lex_keywords() {
    let input = ".byte .org";

    let tests = vec![
        Token::new(TokenType::Byte, Position::new(0, 5), ".byte"),
        Token::new(TokenType::Org, Position::new(6, 10), ".org"),
    ];

    match_against(input, tests)
}

/// Tests the lexer's ability to tokenize identifiers.
#[test]
fn lex_identifiers() {
    let input = "abcde fghij kl_mno OIE ab12CE";

    let tests = vec![
        Token::new(TokenType::Ident, Position::new(0, 5), "abcde"),
        Token::new(TokenType::Ident, Position::new(6, 11), "fghij"),
        Token::new(TokenType::Ident, Position::new(12, 18), "kl_mno"),
        Token::new(TokenType::Ident, Position::new(19, 22), "OIE"),
        Token::new(TokenType::Ident, Position::new(23, 29), "ab12CE"),
    ];

    match_against(input, tests)
}

/// Tests the lexer's ability to tokenize different numbers, including hex, binary, and octal. These are all byte values.
/// Note that the lexer does not support floating point numbers.
#[test]
fn lex_numeric_values() {
    let input = "0x3C 0b1010 0o156 1234 0x 0b 0o";

    let tests = vec![
        Token::new(TokenType::HexNum, Position::new(0, 4), "3C"),
        Token::new(TokenType::BinNum, Position::new(5, 11), "1010"),
        Token::new(TokenType::OctNum, Position::new(12, 17), "156"),
        Token::new(TokenType::DecNum, Position::new(18, 22), "1234"),
        Token::new(TokenType::HexNum, Position::new(23, 25), ""),
        Token::new(TokenType::BinNum, Position::new(26, 28), ""),
        Token::new(TokenType::OctNum, Position::new(29, 31), ""),
    ];

    match_against(input, tests)
}
