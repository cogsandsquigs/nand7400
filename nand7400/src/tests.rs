#![cfg(test)]

use crate::config::Opcode;

use super::*;

/// Test the assembly of known-working code.
#[test]
fn test_assemble_working() {
    let mut assembler = Assembler::new(AssemblerConfig {
        opcodes: vec![
            // Do-nothing opcode.
            Opcode {
                name: "NOP".to_string(),
                num_args: 0,
                binary: 0x00,
            },
            // Add two numbers.
            Opcode {
                name: "ADD".to_string(),
                num_args: 2,
                binary: 0x01,
            },
            // Subtract two numbers.
            Opcode {
                name: "SUB".to_string(),
                num_args: 2,
                binary: 0x02,
            },
            // Jump to an address.
            Opcode {
                name: "JMP".to_string(),
                num_args: 1,
                binary: 0x03,
            },
            // Halt the CPU.
            Opcode {
                name: "HLT".to_string(),
                num_args: 0,
                binary: 0xFF,
            },
        ],
    });

    let test_file = r#"
ADD 0x01 0x02
NOP
JMP 0x01

LABEL1:
    SUB 0x01 0x02
    NOP
    JMP LABEL1

HLT
"#;

    let expected = vec![
        0x01, 0x01, 0x02, // ADD 0x01 0x02
        0x00, // NOP
        0x03, 0x01, // JMP 0x01
        0x02, 0x01, 0x02, // SUB 0x01 0x02
        0x00, // NOP
        0x03, 0x07, // JMP LABEL1
        0xFF, // HLT
    ];

    let actual = assembler.assemble(test_file.to_string()).unwrap();

    assert_eq!(actual, expected,);
}
