pub(crate) mod token;

mod tests;

use num_traits::AsPrimitive;

use self::token::{Token, TokenKind};
use crate::assembler::position::Position;

use super::errors::ParsingError;

/// The `Lexer` is responsible for taking a string of source code and producing a
/// stream of tokens. The `Lexer` is also responsible for keeping track of the current
/// position in the source code, and the current character we are lexing.
///
/// Note that the `Lexer` is not responsible for parsing the source code, it is only
/// responsible for performing lexical analysis - i.e., it is responsible for taking
/// the source code and producing a stream of tokens.
#[derive(Debug, Clone)]
pub struct Lexer {
    /// The source code to lex. A vector of characters, because we need to be able
    /// to parse/index utf-8 characters.
    input: Vec<char>,

    /// The position of the current character we are lexing in the input string. Note that in
    /// the original Monkey implementation, this field is called `position`, but I renamed it
    /// to `current_position` to make it more clear what this is used for.
    current_position: usize,

    /// The position of the next character we are lexing in the input string. Note that in the
    /// original Monkey implementation, this field is called `read_position`, but I renamed it
    /// to `next_position` to make it more clear what this is used for. Also, while you could
    /// remove this field and just use `current_position + 1`, I decided to keep it because it
    /// 1) allows for us to "prime" the lexer without having complicated code, and 2) makes
    /// it easier to understand what is going on.
    next_position: usize,

    /// The current character we are lexing in the input string. This is used as a "storage
    /// space" to keep the current character in, so that we don't have to do annoying things to
    /// get the current character from the input string.
    ch: char,
}

impl Lexer {
    /// Creates a new `Lexer` from the given source code.
    pub fn new(input: &str) -> Self {
        let mut lexer = Self {
            input: input.chars().collect(),
            current_position: 0,
            next_position: 0,
            ch: '\0',
        };

        // Prime the lexer.
        lexer.read_char();

        lexer
    }

    /// Returns the next token in the input string.
    pub fn next_token(&mut self) -> Result<Token, ParsingError> {
        // Skip whitespace characters (not including newlines, as they are significant).
        self.skip_whitespace();

        match self.ch {
            // Parse EOF tokens.
            '\0' => Ok(Token::new(
                TokenKind::Eof,
                Position::new(self.current_position, self.current_position),
                "\0".to_string(),
            )),

            ':' => Ok(self.make_one_char_token(TokenKind::Colon)),
            ';' => Ok(self.make_one_char_token(TokenKind::Semicolon)),
            '#' => Ok(self.make_one_char_token(TokenKind::Hash)),
            '+' => Ok(self.make_one_char_token(TokenKind::Plus)),
            '-' => Ok(self.make_one_char_token(TokenKind::Minus)),

            // Standard POSIX newlines
            '\n' => Ok(self.make_one_char_token(TokenKind::Newline)),

            // \r\n is a newline in Windows, so we need to handle that.
            '\r' if self.peek_char() == '\n' => {
                let prev_pos = self.current_position;
                self.read_char();
                self.read_char();
                Ok(Token::new(
                    TokenKind::Newline,
                    Position::new(prev_pos, self.current_position),
                    "\r\n".to_string(),
                ))
            }

            // \r is a newline in MacOS, so we need to handle that.
            '\r' => {
                println!("{:?}", self);
                Ok(self.make_one_char_token(TokenKind::Newline))
            }

            // The nice thing about rust is that we can match only if the character satisfies
            // some arbitrary constraint. In this case, we are matching if the character is
            // a letter or an underscore. Returning here because we don't need to call `read_char`
            // again, as we already did that in the `read_ident_or_keyword` function, at the end of the
            // loop.
            s if s.is_alphabetic() || s == '_' => {
                let position = self.current_position;
                Ok(Token::from_ident(self.read_ident_or_keyword(), position))
            }

            // Match keywords, which start with a `.` and next character is alphanumeric or underscore.
            '.' if self.peek_char().is_alphanumeric() || self.peek_char() == '_' => {
                let position: usize = self.current_position;
                Token::from_keyword(self.read_ident_or_keyword(), position)
            }

            // Parse integers. Returning here because we don't need to call `read_char` again, as we
            // already did that in the `read_number` function, at the end of the loop.
            s if s.is_ascii_digit() => Ok(self.read_number()),

            _ => {
                let err = Err(ParsingError::UnknownCharacter {
                    character: self.ch,
                    span: Position::new(self.current_position, self.current_position + 1),
                });

                self.read_char();

                err
            }
        }
    }

    /// Collects all the errors that occurred while lexing the input string, and returns a list over them.
    pub fn errors(&mut self) -> Vec<ParsingError> {
        let mut errors = vec![];

        loop {
            if self.ch == '\0' {
                break;
            }

            if let Err(err) = self.next_token() {
                errors.push(err);
            }
        }

        errors
    }
}

impl Lexer {
    /// Reads the next character from the input string, and stores it in the `ch` field. Also
    /// updates the `current_position` and `next_position` fields, and returns the character
    /// that was read, so that it can be used in the calling function. Note that if we are at
    /// the end of the input string, this function will return `\0`, and not update the
    /// `current_position` or `next_position` fields.
    fn read_char(&mut self) -> char {
        // Bounds checking.
        // We don't need to update `next_position` here, because
        // we are at the end of the input string.
        if self.next_position >= self.input.len() {
            self.ch = '\0';

            // Update `current_position` to point to the end of the input string. This is
            // necessary here because we don't want to increment `current_position` when we call
            // `read_char` again, because we are at the end of the input string. However, if we
            // don't do this, it becomes difficult to take slices of the input string near the
            // end of the input string, because we would have to do bounds checking every time
            // we take a slice.
            self.current_position = self.next_position;
        } else {
            // Update `ch` to point to the next character in the input string.
            self.ch = self.input[self.next_position];

            // Update `current_position` and `next_position` to point to the next character.
            // This way of doing things (instead of using `self.current_position += 1`) is better
            // because we can "prime" the lexer by calling `read_char` once, and then we don't
            // need to use complicated code to detect if we are at the beginning of the input.
            self.current_position = self.next_position;
            self.next_position += 1;
        }

        self.ch
    }

    /// Peeks at the next character in the input string, and returns it. This is used when we
    /// encounter a character that could be the start of a two-character token, such as `==`.
    /// This function does not update the lexer's state, so that the next call to `next_token`
    /// will return the same token. Note that if we are at the end of the input string, this
    /// function will return `\0`.
    fn peek_char(&self) -> char {
        // Bounds checking.
        if self.next_position >= self.input.len() {
            '\0'
        } else {
            self.input[self.next_position]
        }
    }

    /// Reads while the function `F` returns `true` on the character value we're reading. Returns
    /// the set of characters that were read.
    fn read_while(&mut self, f: impl Fn(char) -> bool) -> &[char] {
        let position = self.current_position;

        while f(self.ch) {
            self.read_char();
        }

        &self.input[position..self.current_position]
    }

    /// Skips whitespace characters from the input string. This is used when we encounter a
    /// whitespace character, because that means we are lexing whitespace. Note that if you
    /// use this function, you cannot call `read_char` again, because this function already
    /// does that at the last iteration of the loop.
    ///
    /// This skips all whitespace characters except for newlines, because newlines are
    /// significant in the assembly language.
    fn skip_whitespace(&mut self) {
        self.read_while(|c| c.is_whitespace() && (c != '\n' && c != '\r'));
    }

    /// Reads an identifier/keyword from the input string, and returns it as a `String`. This is
    /// used when we encounter a character that is a letter or an underscore, because that means
    /// we are lexing an identifier or keyword. It expects that `ch` is initially alphabetic or an
    /// underscore. Note that if you use this function, you cannot call `read_char` again, because
    /// this function already does that at the last iteration of the loop.
    fn read_ident_or_keyword(&mut self) -> String {
        // Get the position of the first character in the identifier.
        let position = self.current_position;

        // If we encounter a period, we are lexing a keyword, so we need to read the period.
        if self.ch == '.' {
            self.read_char();
        }

        // Keep reading characters until we encounter a character that is not a letter, digit,
        // or underscore.
        while self.ch.is_alphanumeric() || self.ch == '_' {
            self.read_char();
        }

        // Get the identifier from the input string.
        self.input[position..self.current_position].iter().collect()
    }

    /// Reads a number from the input string, and returns it as a `String`. This is used when we encounter
    /// a character that is a digit, because that means we are lexing a number. It expects that `ch` is
    /// initially a digit. Note that if you use this function, you cannot call `read_char` again, because
    /// this function already does that at the last iteration of the loop.
    ///
    /// We can call this for all types of numbers (hex, bin, etc.) because they all start with `0` then some
    /// letter.
    fn read_number(&mut self) -> Token {
        let initial_position = self.current_position;
        // Check if there's an `0x` or `0X` prefix -- parse a hexadecimal number if so.
        let (literal, kind) =
            if self.ch == '0' && self.peek_char() == 'x' || self.peek_char() == 'X' {
                let signifier = self.read_char(); // Consume the `0` character.
                self.read_char(); // Consume the `x` character, and read the number.
                (
                    "0".to_string() + &signifier.to_string() + &self.read_hex_number(),
                    TokenKind::Number,
                )
            }
            // Check if there's an `0o` or `0O` prefix -- parse an octal number if so.
            else if self.ch == '0' && self.peek_char() == 'o' || self.peek_char() == 'O' {
                let signifier = self.read_char(); // Consume the `0` character.
                self.read_char(); // Consume the `x` character, and read the number.
                (
                    "0".to_string() + &signifier.to_string() + &self.read_octal_number(),
                    TokenKind::Number,
                )
            }
            // Check if there's an `0b` or `0B` prefix -- parse a binary number if so.
            else if self.ch == '0' && self.peek_char() == 'b' || self.peek_char() == 'B' {
                let signifier = self.read_char(); // Consume the `0` character.
                self.read_char(); // Consume the `x` character, and read the number.
                (
                    "0".to_string() + &signifier.to_string() + &self.read_binary_number(),
                    TokenKind::Number,
                )
            }
            // Otherwise, parse a decimal number.
            else {
                (self.read_decimal_number(), TokenKind::Number)
            };

        Token::new(
            kind,
            Position::new(initial_position, self.current_position),
            literal,
        )
    }

    /// Reads a hexadecimal number from the input string, and returns it as a `String`
    fn read_hex_number(&mut self) -> String {
        self.read_while(|c| c.is_ascii_hexdigit()).iter().collect()
    }

    /// Reads a binary number from the input string, and returns it as a `String`
    fn read_binary_number(&mut self) -> String {
        self.read_while(|c| c == '0' || c == '1').iter().collect()
    }

    /// Reads an octal number from the input string, and returns it as a `String`
    fn read_octal_number(&mut self) -> String {
        self.read_while(|c| c.is_ascii_digit() && !(c == '8' || c == '9')) // Exclude non-octal digits
            .iter()
            .collect()
    }

    /// Reads a decimal number from the input string, and returns it as a `String`
    fn read_decimal_number(&mut self) -> String {
        self.read_while(|c| c.is_ascii_digit()).iter().collect()
    }

    /// Creates a new token from a single character. Note that you don't have to call `read_char` after
    /// calling this function, because this function already does that.
    fn make_one_char_token(&mut self, kind: TokenKind) -> Token {
        let token = Token::new(
            kind,
            Position::new(self.current_position, self.current_position + 1),
            self.ch.to_string(),
        );

        self.read_char();

        token
    }
}
