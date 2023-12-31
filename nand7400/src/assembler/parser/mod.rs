pub mod ast;
pub mod errors;
pub mod lexer;

mod tests;

use std::num::{IntErrorKind, ParseIntError};

use self::{
    ast::{Argument, ArgumentKind, Instruction, Keyword},
    errors::ParsingError,
    lexer::{
        token::{Token, TokenKind},
        Lexer,
    },
};
use crate::assembler::{
    parser::ast::{InstructionKind, Label},
    position::Position,
};
use ast::Ast;
use num_traits::{AsPrimitive, FromPrimitive, Num, Signed, Unsigned};

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
        // Developer notes: The main loop is just a loop that calls these parsing functions, inserts instructions, and then
        // returns the AST when it's done. Sub-parsers are required to update the symbol table as necessary.

        // Loop until we finish parsing.
        loop {
            // Match on the token, and then parse it.
            let instruction = match self.current_token.kind {
                // If the token is an EOF, then we're done parsing.
                TokenKind::Eof => return Ok(self.ast),

                // If the token is a newline while parsing a file, then we skip it. We only care about these when
                // parsing an opcode or keyword.
                TokenKind::Newline => {
                    self.read_token()?;
                    continue;
                }

                // If we reach a comment, we consume it in entirety.
                TokenKind::Semicolon => {
                    self.read_token_unchecked();

                    while !matches!(self.current_token.kind, TokenKind::Newline | TokenKind::Eof) {
                        self.read_token_unchecked();
                    }

                    continue;
                }

                // If the token is a identifier, then we have either a label or opcode.
                TokenKind::Ident => self.parse_label_or_opcode()?,

                // If the token is a keyword, then we have a keyword instruction.
                TokenKind::Keyword => {
                    let current = self.current_token.clone();

                    self.read_token()?;

                    self.parse_keyword(current)?
                }

                _ => todo!(),
            };

            self.ast.instructions.push(instruction.clone());
            self.next_mem_location += instruction.binary_len();
        }
    }
}

impl Parser {
    /// Gets the next token from the lexer.
    fn read_token(&mut self) -> Result<Token, ParsingError> {
        let token = self.read_token_unchecked();

        match token.kind {
            // If the token is ok, then we return it raw.
            TokenKind::Invalid => Err(ParsingError::UnknownCharacter {
                character: token.literal,
                span: token.position,
            }),

            // If the token is an error, Then we return the error as-is.
            _ => Ok(token),
        }
    }

    /// Gets the next token from the lexer, without regard to illegality.
    fn read_token_unchecked(&mut self) -> Token {
        let token = self.lexer.next_token();
        self.current_token = token.clone();

        token
    }

    /// Parse either a label or an opcode instruction.
    fn parse_label_or_opcode(&mut self) -> Result<Instruction, ParsingError> {
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
    fn parse_label(&mut self, label_token: Token) -> Result<Instruction, ParsingError> {
        let label_name: Label = label_token.literal;

        let instruction = Instruction::new(
            InstructionKind::Label(label_name.clone()),
            label_token.position.join(&self.current_token.position), // Include the colon in the instruction span.
            label_token.position,
        );

        self.ast.symbols.insert(label_name, self.next_mem_location);

        // Consume the colon.
        self.read_token()?;

        Ok(instruction)
    }

    /// Parse a single opcode from tokens. We expect that the current token is *not* the opcode, but the token after it;
    /// and that `opcode_token` is the token of the opcode.
    fn parse_opcode(&mut self, opcode_token: Token) -> Result<Instruction, ParsingError> {
        let (arguments, current_pos) = self.parse_argument_list::<u8, i8>(opcode_token.position)?;

        let opcode = Instruction::new(
            InstructionKind::Opcode {
                mnemonic: opcode_token.literal,
                arguments,
            },
            opcode_token.position.join(&current_pos),
            opcode_token.position,
        );

        // Match on the token, and then parse it.
        match self.current_token.kind {
            // If the token is an EOF or newline, then we're done parsing.
            TokenKind::Eof | TokenKind::Newline => Ok(opcode),

            // Otherwise, we have an error.
            _ => todo!("{:?}", self.current_token),
        }
    }

    /// Parse a single keyword from tokens. We expect that the current token is *not* the keyword, but the token after it;
    /// and that `keyword_token` is the token of the keyword.
    fn parse_keyword(&mut self, keyword_token: Token) -> Result<Instruction, ParsingError> {
        let (arguments, current_pos) =
            self.parse_argument_list::<u16, i16>(keyword_token.position)?;

        let keyword_kind = match keyword_token.literal.to_ascii_lowercase().as_str() {
            ".byte" => Keyword::Byte,
            ".org" => {
                // Set the current memory address to the first argument, so labels end up in the correct place.
                match arguments.first() {
                    Some(Argument {
                        kind:
                            ArgumentKind::ImmediateNumber(number) | ArgumentKind::IndirectNumber(number), // Both numbers and labels are valid for the first argument, and are treated as immediate values.
                        ..
                    }) => self.next_mem_location = *number,
                    _ => unreachable!(),
                }

                Keyword::Org
            }

            _ => {
                return Err(ParsingError::KeywordDNE {
                    mnemonic: keyword_token.literal,
                    span: keyword_token.position,
                })
            }
        };

        let keyword = Instruction::new(
            InstructionKind::Keyword {
                keyword: keyword_kind,
                arguments,
            },
            keyword_token.position.join(&current_pos),
            keyword_token.position,
        );

        // Match on the token, and then parse it.
        match self.current_token.kind {
            // If the token is an EOF or newline, then we're done parsing.
            TokenKind::Eof | TokenKind::Newline => Ok(keyword),

            // Otherwise, we have an error.
            _ => todo!("{:?}", self.current_token),
        }
    }

    /// Parse a list of arguments from tokens. `pos` is the position of the token that calls the arguments. It returns
    /// the list of arguments and the position of the last token parsed.
    fn parse_argument_list<U, V>(
        &mut self,
        pos: Position,
    ) -> Result<(Vec<Argument<U>>, Position), ParsingError>
    where
        U: 'static + Num<FromStrRadixErr = ParseIntError> + Unsigned + FromPrimitive + Copy,
        V: Num<FromStrRadixErr = ParseIntError> + Signed + AsPrimitive<U>,
    {
        let mut arguments = vec![];
        let mut current_pos = pos;

        // Parse all the arguments.
        while !matches!(self.current_token.kind, TokenKind::Newline | TokenKind::Eof) {
            match self.current_token.kind {
                TokenKind::Ident => {
                    let label_name: Label = self.current_token.literal.clone();

                    // Insert the label.
                    arguments.push(Argument {
                        kind: ArgumentKind::Label(label_name),
                        span: self.current_token.position,
                    });

                    current_pos = current_pos.join(&self.current_token.position);

                    // Consume the label name.
                    self.read_token()?;
                }

                _ => {
                    let arg = self.parse_numeric_argument::<U, V>()?;

                    current_pos = current_pos.join(&arg.span);
                    arguments.push(arg);
                }
            }
        }

        Ok((arguments, current_pos))
    }

    /// Parse a single numeric argument from tokens. We expect that the current token is a number or a `#`. `U` is the
    /// unsigned type of number the argument is, and `V` is the signed variang the number is parsed as if it's signed.
    fn parse_numeric_argument<U, V>(&mut self) -> Result<Argument<U>, ParsingError>
    where
        U: 'static + Num<FromStrRadixErr = ParseIntError> + Unsigned + FromPrimitive + Copy,
        V: Num<FromStrRadixErr = ParseIntError> + Signed + AsPrimitive<U>,
    {
        // Match on the token, and then parse it.
        match self.current_token.kind {
            // If it's a number, then we consume it and go back to parsing the file. Note that numbers without a '#' are
            // indirection, and numbers with a '#' are immediate.
            TokenKind::Number => {
                let literal = self.current_token.literal.clone();
                let pos = self.current_token.position;

                // Parse the number.
                let number: U = parse_number(&literal, pos)?;

                // Consume the number.
                self.read_token()?;

                Ok(Argument {
                    kind: ArgumentKind::IndirectNumber(number),
                    span: pos,
                })
            }

            // If it's positive, then we consume the `+` and then parse the number.
            TokenKind::Plus => {
                // Get the `+` position so we join it with the number.
                let plus_pos = self.current_token.position;

                // Consume the `+`.
                self.read_token()?;

                // Now, read the number
                let literal = self.current_token.literal.clone();
                let pos = plus_pos.join(&self.current_token.position);

                // Parse the number.
                let number: V = parse_number(&literal, pos)?;

                // Consume the number.
                self.read_token()?;

                Ok(Argument {
                    kind: ArgumentKind::IndirectNumber(number.as_()),
                    span: pos,
                })
            }

            // If it's negative, then we consume the `-` and then parse the number.
            TokenKind::Minus => {
                // Get the `-` position so we join it with the number.
                let neg_pos = self.current_token.position;

                // Consume the `-`.
                self.read_token()?;

                // Now, read the number
                let literal = self.current_token.literal.clone();
                let pos = neg_pos.join(&self.current_token.position);

                // Parse the number.
                let number: V = parse_number(&literal, pos)?;

                // Consume the number.
                self.read_token()?;

                Ok(Argument {
                    kind: ArgumentKind::IndirectNumber((-number).as_()),
                    span: pos,
                })
            }

            // If it's a `#`, then we consume it and then parse the as a direct/immediate number. Note that `#` is only
            // used for immediate numbers, and not indirection.
            TokenKind::Hash => {
                // Get the `#` position so we join it with the number.
                let hash_pos = self.current_token.position;

                // Consume the `#`.
                self.read_token()?;

                // Parse the number with `parse_number`, and then return but change the value to an indirection.
                let arg = self.parse_numeric_argument::<U, V>()?;

                match arg.kind {
                    ArgumentKind::ImmediateNumber(number)
                    | ArgumentKind::IndirectNumber(number) => Ok(Argument {
                        kind: ArgumentKind::ImmediateNumber(number),
                        span: hash_pos.join(&arg.span),
                    }),

                    _ => unreachable!(),
                }
            }

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
fn parse_number<T>(literal: &str, span: Position) -> Result<T, ParsingError>
where
    T: Num<FromStrRadixErr = ParseIntError>,
{
    match &literal[..2] {
        "0x" | "0X" => T::from_str_radix(&literal[2..], 16),
        "0b" | "0B" => T::from_str_radix(&literal[2..], 2),
        "0o" | "0O" => T::from_str_radix(&literal[2..], 8),
        _ => T::from_str_radix(literal, 10),
    }
    .map_err(|err| match err.kind() {
        // If the literal is too large, then we should report an error.
        IntErrorKind::PosOverflow => ParsingError::Overflow {
            literal: literal.to_string(),
            span,
        },

        // Ditto if a literal is too small
        IntErrorKind::NegOverflow => ParsingError::Underflow {
            literal: literal.to_string(),
            span,
        },

        // If the literal is empty, then we should report an error.
        IntErrorKind::Empty => ParsingError::EmptyLiteral { span },

        // Check if the digits are invalid.
        // TODO: Parse a general number w/o respect for digits and then check if the digits are invalid, instead
        // of filtering out digits in parse-time.
        IntErrorKind::InvalidDigit => panic!("Invalid digit!: {}", literal),

        // Unreachable because i8s and u8s allow for 0 as a valid value.
        IntErrorKind::Zero => unreachable!("i8s and u8s should allow 0 as a value!"),

        // Unreachable because there should be no more errors to consider.
        _ => unreachable!("There should be no more errors to consider!"),
    })
}
