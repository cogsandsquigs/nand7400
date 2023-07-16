#![cfg(test)]

use super::{AssemblyParser, Rule};
use pest::{consumes_to, parses_to};

/// Test the parsing of a whole file.
#[test]
fn test_parse_file() {
    parses_to! {
        parser: AssemblyParser,
        input:  "nop\n",
        rule:   Rule::File,
        tokens: [
            File(0, 4, [
                Instruction(0, 3, [
                    Identifier(0, 3),
                ]),
                EOI(4, 4),
            ])
        ]
    };

    parses_to! {
        parser: AssemblyParser,
        input: "

            // There's whitespace at the beginning and end to test the parsing of extraneous newlines/whitespace!
            
            /*
                This is a multi-line comment! anything can go here!
                Yay!
            */

            nop
            lda 0xCA
            jmp LABEL

            LABEL:
                nop
            ",
        rule:   Rule::File,
        tokens: [
            File(0, 360, [
                Instruction(261, 264, [
                    Identifier(261, 264),
                ]),
                Instruction(277, 285, [
                    Identifier(277, 280),
                    Literal(281, 285),
                ]),
                Instruction(298, 307, [
                    Identifier(298, 301),
                    Identifier(302, 307),
                ]),
                Label(321, 327, [
                    Identifier(321, 326),
                    Colon(326, 327),
                ]),
                Instruction(344, 347, [
                    Identifier(344, 347),
                ]),
                EOI(360, 360),
            ])
        ]
    };
}

/// Test the parsing of positive and negative values.
#[test]
fn test_parse_positives_negatives() {
    parses_to! {
        parser: AssemblyParser,
        input: "add +0x01 -0x02 0x03",
        rule:   Rule::File,
        tokens: [
            File(0, 20, [
                Instruction(0, 20, [
                    Identifier(0, 3),
                    Literal(4, 9),
                    Literal(10, 15),
                    Literal(16, 20),
                ]),
                EOI(20, 20),
            ])
        ]
    }
}

/// Test the parsing of a label.
#[test]
fn test_parse_label() {
    parses_to! {
        parser: AssemblyParser,
        input:  "foo:\n",
        rule:   Rule::Label,
        tokens: [
            Label(0, 4, [
                Identifier(0, 3),
                Colon(3, 4),
            ])
        ]
    };

    parses_to! {
        parser: AssemblyParser,
        input:  "foo_bar_baz:",
        rule:   Rule::Label,
        tokens: [
            Label(0, 12, [
                Identifier(0, 11),
                Colon(11, 12),
            ])
        ]
    };
}

/// Test the parsing of an instruction.
#[test]
fn test_parse_instruction() {
    parses_to! {
        parser: AssemblyParser,
        input:  "nop\n",
        rule:   Rule::Instruction,
        tokens: [
            Instruction(0, 3, [
                Identifier(0, 3),
            ])
        ]
    };

    parses_to! {
        parser: AssemblyParser,
        input:  "JmP 0x1234",
        rule:   Rule::Instruction,
        tokens: [
            Instruction(0, 10, [
                Identifier(0, 3),
                Literal(4, 10),
            ])
        ]
    };

    parses_to! {
        parser: AssemblyParser,
        input:  "ADD 0x1234 abc 456\n",
        rule:   Rule::Instruction,
        tokens: [
            Instruction(0, 18, [
                Identifier(0, 3),
                Literal(4, 10),
                Identifier(11, 14),
                Literal(15, 18),
            ])
        ]
    };
}

/// Test parsing an identifier.
#[test]
fn test_parse_identifier() {
    parses_to! {
        parser: AssemblyParser,
        input:  "foo",
        rule:   Rule::Identifier,
        tokens: [
            Identifier(0, 3)
        ]
    };

    parses_to! {
        parser: AssemblyParser,
        input:  "fo0_b4r",
        rule:   Rule::Identifier,
        tokens: [
            Identifier(0, 7)
        ]
    };

    parses_to! {
        parser: AssemblyParser,
        input:  "foo_bar_baz",
        rule:   Rule::Identifier,
        tokens: [
            Identifier(0, 11)
        ]
    };

    parses_to! {
        parser: AssemblyParser,
        input:  "foo_bar_baz_",
        rule:   Rule::Identifier,
        tokens: [
            Identifier(0, 12)
        ]
    };

    parses_to! {
        parser: AssemblyParser,
        input:  "foo_bar_baz_123",
        rule:   Rule::Identifier,
        tokens: [
            Identifier(0, 15)
        ]
    };

    parses_to! {
        parser: AssemblyParser,
        input:  "foo_bar_baz_123_",
        rule:   Rule::Identifier,
        tokens: [
            Identifier(0, 16)
        ]
    };

    parses_to! {
        parser: AssemblyParser,
        input:  "foo_bar_baz_123_456",
        rule:   Rule::Identifier,
        tokens: [
            Identifier(0, 19)
        ]
    };
}

/// Test parsing a literal value.
#[test]
fn test_parse_literal() {
    parses_to! {
        parser: AssemblyParser,
        input:  "012345689",
        rule:   Rule::Literal,
        tokens: [
            Literal(0, 9)
        ]
    };

    parses_to! {
        parser: AssemblyParser,
        input:  "0x0123456789ABCDEF",
        rule:   Rule::Literal,
        tokens: [
            Literal(0, 18)
        ]
    };

    parses_to! {
        parser: AssemblyParser,
        input:  "0X0123456789abcdef",
        rule:   Rule::Literal,
        tokens: [
            Literal(0, 18)
        ]
    };

    parses_to! {
        parser: AssemblyParser,
        input:  "0b101",
        rule:   Rule::Literal,
        tokens: [
            Literal(0, 5)
        ]
    };

    parses_to! {
        parser: AssemblyParser,
        input:  "0o01234567",
        rule:   Rule::Literal,
        tokens: [
            Literal(0, 10)
        ]
    };

    // Check if it can parse positives
    parses_to! {
        parser: AssemblyParser,
        input:  "+0x0123456789abcdef",
        rule:   Rule::Literal,
        tokens: [
            Literal(0, 19)
        ]
    };

    // Check if it can parse negatives
    parses_to! {
        parser: AssemblyParser,
        input:  "-0x0123456789abcdef",
        rule:   Rule::Literal,
        tokens: [
            Literal(0, 19)
        ]
    };
}
