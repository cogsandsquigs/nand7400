#![cfg(test)]

use super::*;
use std::collections::HashMap;

#[macro_use]
mod macros {
    /// Tests a list of tokens against the lexer's output.
    macro_rules! parses_as {
        ($parse_func:ident$(::<$($x:ty),+ $(,)?>)?, $input:expr, $ast:expr $(,)?) => {
            #[allow(unused_mut)]

            let mut parser = match crate::assembler::parser::Parser::new($input) {
                Ok(parser) => parser,
                Err(err) => panic!("Failed to create parser: {}", err),
            };


            let parsed_ast = match parser.$parse_func$(::<$($x),+>)?() {
                Ok(parser) => parser,
                Err(err) => {
                    return Err(miette::Report::new(err.clone()).with_source_code($input));
                },
            };

            assert_eq!(parsed_ast, $ast);
        };
    }
}

/// Test EOF parsing, and make sure that EOFs return the AST unchanged.
#[test]
fn parse_eof() -> miette::Result<()> {
    parses_as!(parse, "", Ast::empty());

    parses_as!(parse, "\n\n\n", Ast::empty());

    Ok(())
}

/// Test the parsing of a comment.
#[test]
fn parse_comment() -> miette::Result<()> {
    parses_as!(parse, "; this is a comment", Ast::empty());
    parses_as!(parse, "; this is a comment\n\n\n", Ast::empty());

    parses_as!(
        parse,
        "label: ;testing testing 1 2 3",
        Ast {
            instructions: vec![Instruction {
                kind: InstructionKind::Label("label".to_string()),
                instruction_span: Position::new(0, 6),
                token_span: Position::new(0, 5),
            }],
            symbols: HashMap::from([("label".to_string(), 0)]),
        },
    );

    Ok(())
}

/// Test the parsing of a label.
#[test]
fn parse_label() -> miette::Result<()> {
    parses_as!(
        parse,
        "label:",
        Ast {
            instructions: vec![Instruction {
                kind: InstructionKind::Label("label".to_string()),
                instruction_span: Position::new(0, 6),
                token_span: Position::new(0, 5),
            }],
            symbols: HashMap::from([("label".to_string(), 0)]),
        },
    );

    parses_as!(
        parse,
        "asdf123: \n   \r\n \n ",
        Ast {
            instructions: vec![Instruction {
                kind: InstructionKind::Label("asdf123".to_string()),
                instruction_span: Position::new(0, 8),
                token_span: Position::new(0, 7),
            }],
            symbols: HashMap::from([("asdf123".to_string(), 0)]),
        },
    );

    Ok(())
}

/// Test the parsing of opcodes.
#[test]
fn parse_opcode() -> miette::Result<()> {
    parses_as!(
        parse,
        "NOP",
        Ast {
            instructions: vec![Instruction {
                kind: InstructionKind::Opcode {
                    mnemonic: "NOP".to_string(),
                    arguments: vec![]
                },
                instruction_span: Position::new(0, 3),
                token_span: Position::new(0, 3),
            }],
            symbols: HashMap::new(),
        },
    );

    parses_as!(
        parse,
        "NOP \r\n\n   \n ",
        Ast {
            instructions: vec![Instruction {
                kind: InstructionKind::Opcode {
                    mnemonic: "NOP".to_string(),
                    arguments: vec![]
                },
                instruction_span: Position::new(0, 3),
                token_span: Position::new(0, 3),
            }],
            symbols: HashMap::new(),
        },
    );

    Ok(())
}

/// Test opcodes with arguments, and make sure that the arguments are parsed correctly.
#[test]
fn parse_opcode_with_arguments() -> miette::Result<()> {
    parses_as!(
        parse,
        "test1 123 #45 0x67",
        Ast {
            instructions: vec![Instruction {
                kind: InstructionKind::Opcode {
                    mnemonic: "test1".to_string(),
                    arguments: vec![
                        Argument {
                            kind: ArgumentKind::IndirectNumber(123),
                            span: Position::new(6, 9),
                        },
                        Argument {
                            kind: ArgumentKind::ImmediateNumber(45),
                            span: Position::new(10, 13),
                        },
                        Argument {
                            kind: ArgumentKind::IndirectNumber(0x67),
                            span: Position::new(14, 18),
                        },
                    ]
                },
                instruction_span: Position::new(0, 18),
                token_span: Position::new(0, 5),
            }],
            symbols: HashMap::new(),
        },
    );

    Ok(())
}

/// Test the parsing of keywords (`.byte`, `.org`, etc.).
#[test]
fn parse_keyword() -> miette::Result<()> {
    parses_as!(
        parse,
        ".byte 0x12",
        Ast {
            instructions: vec![Instruction {
                kind: InstructionKind::Keyword {
                    keyword: Keyword::Byte,
                    arguments: vec![Argument {
                        kind: ArgumentKind::IndirectNumber(0x12),
                        span: Position::new(6, 10),
                    }]
                },
                instruction_span: Position::new(0, 10),
                token_span: Position::new(0, 5),
            }],
            symbols: HashMap::new(),
        },
    );

    parses_as!(
        parse,
        ".org 0x123",
        Ast {
            instructions: vec![Instruction {
                kind: InstructionKind::Keyword {
                    keyword: Keyword::Org,
                    arguments: vec![Argument {
                        kind: ArgumentKind::IndirectNumber(0x123),
                        span: Position::new(5, 10),
                    }]
                },
                instruction_span: Position::new(0, 10),
                token_span: Position::new(0, 4),
            }],
            symbols: HashMap::new(),
        },
    );

    Ok(())
}

/// Test if the '.org' keyword affects the memory address of labels correctly.
#[test]
fn parse_org_label_addrs() -> miette::Result<()> {
    parses_as!(
        parse,
        "label1:\n.org 0x123\nlabel2:",
        Ast {
            instructions: vec![
                Instruction {
                    kind: InstructionKind::Label("label1".to_string()),
                    instruction_span: Position::new(0, 7),
                    token_span: Position::new(0, 6),
                },
                Instruction {
                    kind: InstructionKind::Keyword {
                        keyword: Keyword::Org,
                        arguments: vec![Argument {
                            kind: ArgumentKind::IndirectNumber(0x123),
                            span: Position::new(13, 18),
                        }]
                    },
                    instruction_span: Position::new(8, 18),
                    token_span: Position::new(8, 12),
                },
                Instruction {
                    kind: InstructionKind::Label("label2".to_string()),
                    instruction_span: Position::new(19, 26),
                    token_span: Position::new(19, 25),
                },
            ],
            symbols: HashMap::from([("label1".to_string(), 0), ("label2".to_string(), 0x123)]),
        },
    );

    Ok(())
}

/// Test the parsing of a number, both indirect, direct, positive, and negative.
#[test]
fn parse_number_prefixes() -> miette::Result<()> {
    parses_as!(
        parse_numeric_argument::<u8, i8>,
        "123",
        Argument {
            kind: ArgumentKind::IndirectNumber(123),
            span: Position::new(0, 3),
        },
    );

    parses_as!(
        parse_numeric_argument::<u8, i8>,
        "+123",
        Argument {
            kind: ArgumentKind::IndirectNumber(123),
            span: Position::new(0, 4),
        },
    );

    parses_as!(
        parse_numeric_argument::<u8, i8>,
        "-123",
        Argument {
            kind: ArgumentKind::IndirectNumber(-123_i8 as u8),
            span: Position::new(0, 4),
        }
    );

    parses_as!(
        parse_numeric_argument::<u8, i8>,
        "#123",
        Argument {
            kind: ArgumentKind::ImmediateNumber(123),
            span: Position::new(0, 4),
        },
    );

    parses_as!(
        parse_numeric_argument::<u8, i8>,
        "#+123",
        Argument {
            kind: ArgumentKind::ImmediateNumber(123),
            span: Position::new(0, 5),
        },
    );

    parses_as!(
        parse_numeric_argument::<u8, i8>,
        "#-123",
        Argument {
            kind: ArgumentKind::ImmediateNumber(-123_i8 as u8),
            span: Position::new(0, 5),
        }
    );

    Ok(())
}

/// Test the parsing of numbers with different bases.
#[test]
fn parse_number_bases() -> miette::Result<()> {
    parses_as!(
        parse_numeric_argument::<u8, i8>,
        "0b101",
        Argument {
            kind: ArgumentKind::IndirectNumber(0b101),
            span: Position::new(0, 5),
        },
    );

    parses_as!(
        parse_numeric_argument::<u8, i8>,
        "0o123",
        Argument {
            kind: ArgumentKind::IndirectNumber(0o123),
            span: Position::new(0, 5),
        },
    );

    parses_as!(
        parse_numeric_argument::<u8, i8>,
        "0xFE",
        Argument {
            kind: ArgumentKind::IndirectNumber(0xFE),
            span: Position::new(0, 4),
        },
    );

    Ok(())
}

/// Test the parsing of an entire program.
#[test]
fn parse_program() -> miette::Result<()> {
    let program: &str = "; There's whitespace at the beginning and end to test the parsing of extraneous newlines/whitespace!\n\
                         ; Here's one comment\n\
                         ; Here's another comment\n\
                         \n\
                         ; Now for some *real* code!\n\
                         .OrG 0x10\n\
                         lda 0x09\n\
                         jmp LABEL\n\
                         \n\
                         LABEL:\n\
                             .bYtE 0xFF\n\
                             add 0x01 0x02 0x03\n\
                             hlt";

    parses_as!(
        parse,
        program,
        Ast {
            instructions: vec![],
            symbols: HashMap::from([("LABEL".to_string(), 21)]),
        }
    );

    Ok(())
}
