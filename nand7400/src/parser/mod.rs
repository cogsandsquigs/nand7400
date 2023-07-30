pub mod ast;
pub mod errors;

mod lexer;
mod tests;

use std::num::IntErrorKind;

use self::{
    ast::{Argument, ArgumentKind, Instruction},
    errors::ParsingError,
    lexer::{
        token::{Token, TokenKind},
        Lexer,
    },
};
use crate::{
    parser::ast::{InstructionKind, Label},
    position::Position,
};
use ast::Ast;
use num_traits::{Num, Signed, Unsigned};

/// The parser type, used to parse the source code.
pub struct Parser {
    /// The lexer used to lex the source code.
    lexer: Lexer,

    /// The current AST being built.
    ast: Ast,

    /// The memory location of the next instruction.
    next_mem_location: u16,

    /// The current token type.
    current_token: Token,
}

impl Parser {
    /// Create a new parser from some source code.
    pub fn new(source: &str) -> Result<Self, ParsingError> {
        let mut parser = Self {
            lexer: Lexer::new(source),
            ast: Ast::empty(),
            next_mem_location: 0, // Start at location 0x0000.
            current_token: Token {
                kind: TokenKind::Eof,
                position: Position::new(0, 0),
                literal: String::new(),
            },
        };

        // Get the first token, and remove the invalid placeholder one.
        parser.current_token = parser.read_token()?;

        Ok(parser)
    }

    /// Parses and returns the AST.
    pub fn parse(mut self) -> Result<Ast, ParsingError> {
        // Developer notes: All parsing sub-functions have the responsibility of inserting the instruction into the AST,
        // updating the symbol table and memory location, and consuming the next token (if necessary). The main loop is just
        // a loop that calls these functions, and then returns the AST when it's done.

        // Loop until we finish parsing.
        loop {
            // Match on the token, and then parse it.
            match self.current_token.kind {
                // If the token is an EOF, then we're done parsing.
                TokenKind::Eof => return Ok(self.ast),

                // If the token is a newline, then we skip it. We only care about these when parsing an opcode or keyword.
                TokenKind::Newline => {
                    self.read_token()?;
                }

                // If the token is a identifier, then we have either a label or opcode.
                TokenKind::Ident => self.parse_label_or_opcode()?,

                // If the token is a keyword, then we have a keyword instruction.
                TokenKind::Keyword => {
                    let instruction: Instruction = todo!();
                    self.next_mem_location += instruction.binary_len();
                    self.ast.instructions.push(instruction);
                }

                _ => todo!(),
            };
        }
    }
}

impl Parser {
    /// Gets the next token from the lexer.
    fn read_token(&mut self) -> Result<Token, ParsingError> {
        match self.lexer.next_token() {
            // If the token is ok, then we return it raw.
            Ok(token) => {
                self.current_token = token.clone();

                Ok(token)
            }

            // If the token is an error, Then we return the error as-is.
            Err(err) => Err(err),
        }
    }

    /// Parse either a label or an opcode instruction.
    fn parse_label_or_opcode(&mut self) -> Result<(), ParsingError> {
        let current = self.current_token.clone();

        match self.read_token()?.kind {
            // If the next token is a we consume it and go back to parsing the file.
            TokenKind::Colon => self.parse_label(current),

            // If the next token is anything else, then we parse an opcode. Errors are handled there.
            _ => self.parse_opcode(current),
        }
    }

    /// Parse a single label from tokens. We expect that the current token is a colon (":"), and that `label_token` is the
    /// token of the label. We can then safely consume the colon, parse the label, and go back to parsing the file.
    fn parse_label(&mut self, label_token: Token) -> Result<(), ParsingError> {
        let label_name: Label = label_token.literal;

        let instruction = Instruction::new(
            InstructionKind::Label(label_name.clone()),
            label_token.position,
        );

        self.ast.instructions.push(instruction.clone());
        self.ast.symbols.insert(label_name, self.next_mem_location);

        // Consume the colon.
        self.read_token()?;

        Ok(())
    }

    /// Parse a single opcode from tokens. We expect that the current token is *not* the opcode, but the token after it;
    /// and that `opcode_token` is the token of the opcode.
    fn parse_opcode(&mut self, opcode_token: Token) -> Result<(), ParsingError> {
        let arguments = vec![];

        todo!();

        let opcode = Instruction::new(
            InstructionKind::Opcode {
                mnemonic: opcode_token.literal,
                arguments,
            },
            opcode_token.position.join(&self.current_token.position),
        );

        self.ast.instructions.push(opcode);
        self.next_mem_location += opcode.binary_len();

        // Consume the last argument, which is either a newline or EOF.
        self.read_token()?;

        // Match on the token, and then parse it.
        match self.current_token.kind {
            // If the token is an EOF or newline, then we're done parsing.
            TokenKind::Eof | TokenKind::Newline => return Ok(()),

            // Otherwise, we have an error.
            _ => todo!(),
        };

        Ok(())
    }

    /// Parse a single numeric argument from tokens. We expect that the current token is a number or a `#`. `U` is the
    /// unsigned type of number the argument is, and `V` is the signed variang the number is parsed as if it's signed.
    fn parse_numeric_argument<U, V>(&mut self) -> Result<Argument<U>, ParsingError>
    where
        U: Num + Unsigned + From<V>,
        V: Num + Signed + Into<U>,
        U::FromStrRadixErr: std::fmt::Debug,
        V::FromStrRadixErr: std::fmt::Debug,
    {
        // Match on the token, and then parse it.
        match self.current_token.kind {
            // If it's a number, then we consume it and go back to parsing the file. Note that numbers without a '#' are
            // indirection, and numbers with a '#' are immediate.
            TokenKind::Number => todo!(),

            // If it's positive, then we consume the `+` and then parse the number.
            TokenKind::Plus => todo!(),

            // If it's negative, then we consume the `-` and then parse the number.
            TokenKind::Minus => todo!(),

            // If it's a `#`, then we consume it and then parse the as a direct/immediate number. Note that `#` is only
            // used for immediate numbers, and not indirection.
            TokenKind::Hash => todo!(),

            _ => Err(ParsingError::Unexpected {
                expected: vec![
                    TokenKind::Number,
                    TokenKind::Plus,
                    TokenKind::Minus,
                    TokenKind::Hash,
                ],
                found: self.current_token.kind,
                span: self.current_token.position,
            }),
        }
    }
}

/// Parse a number, *not* a numeric argument. This returns the number as a `T`, and is used for parsing arguments.
/// Note that this does *not* call `read_token`, because it's used in `parse_numeric_argument`, which does that for us.
/// It expects that `literal` does *not* contain the numeric prefix (e.g. "0x", "0b", "0o").
fn parse_number<T>(literal: &str) -> Result<T, T::FromStrRadixErr>
where
    T: Num,
{
    match literal {
        "0x" | "0X" => T::from_str_radix(&literal[2..], 16),
        "0b" | "0B" => T::from_str_radix(&literal[2..], 2),
        "0o" | "0O" => T::from_str_radix(&literal[2..], 8),
        _ => T::from_str_radix(literal, 10),
    }
}
