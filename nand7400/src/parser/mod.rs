pub mod ast;
pub mod errors;

mod tests;

use self::errors::ParsingError;
use crate::{
    lexer::{
        token::{Token, TokenKind},
        Lexer,
    },
    position::Position,
};
use ast::Ast;
use itertools::Itertools;

/// The parser type, used to parse the source code.
pub struct Parser {
    /// The lexer used to lex the source code.
    lexer: Lexer,

    /// The current AST being built.
    ast: Ast,

    /// The current location in memory.
    mem_location: u16,

    /// The current token type.
    current_token: Token,
}

impl Parser {
    /// Create a new parser from some source code.
    pub fn new(source: &str) -> Result<Self, Vec<ParsingError>> {
        let mut parser = Self {
            lexer: Lexer::new(source),
            ast: Ast::empty(),
            mem_location: 0, // Start at location 0x0000.
            current_token: Token {
                kind: TokenKind::Eof,
                position: Position::new(0, 0),
                literal: String::new(),
            },
        };

        // Get the first token, and remove the invalid placeholder one.
        parser.current_token = parser.next_token()?;

        Ok(parser)
    }

    /// Parses and returns the AST.
    pub fn parse(mut self) -> Result<Ast, Vec<ParsingError>> {
        // Loop until we finish parsing.
        loop {
            // Match on the token, and then parse it.
            match self.current_token.kind {
                // If the token is an EOF, then we're done parsing.
                TokenKind::Eof => return Ok(self.ast),

                _ => todo!(),
            }
        }
    }
}

impl Parser {
    /// Gets the next token from the lexer.
    fn next_token(&mut self) -> Result<Token, Vec<ParsingError>> {
        match self.lexer.next_token() {
            // If the token is ok, then we return it raw.
            Ok(token) => Ok(token),

            // If the token is an error, then we return *all* the errors the lexer reports.
            Err(err) => {
                let mut errors = self
                    .lexer
                    .errors()
                    .iter()
                    .map(|err| ParsingError::Lexing(err.clone()))
                    .collect_vec();

                // Add back the original error.
                errors.push(ParsingError::Lexing(err));

                Err(errors)
            }
        }
    }

    /// Parse a single label from tokens. We expect that
    fn parse_label(&mut self) -> Result<(), ParsingError> {
        todo!()
    }
}
