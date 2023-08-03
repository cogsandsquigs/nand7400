#![cfg(test)]

use super::*;
use crate::assembler::config::Opcode;

/// Test the `.byte` and `.org` keywords.
#[test]
fn assemble_keywords() {
    let source = ".byte 0x00 0x01\n\
                        .byte 0x01\n\
                        .byte 0x02\n\
                        .byte 0x03\n\
                        .org 0x10\n\
                        .byte 0x04\n\
                        .byte 0x05\n";

    let mut assembler = Assembler::new(AssemblerConfig { opcodes: vec![] });
    let result = assembler.assemble(source).unwrap();

    assert_eq!(
        result,
        vec![
            0x00, 0x01, 0x01, 0x02,
            0x03, // .byte 0x00 0x01\n.byte 0x01\n.byte 0x02\n.byte 0x03\n
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // .org 0x10\n
            0x04, 0x05, // .byte 0x04\n.byte 0x05\n
        ]
    );
}

/// Test opcodes with no arguments (i.e. implied opcodes)
#[test]
fn assemble_opcodes_implied() {
    let source = "nop\n\
                        hlt\n\
                        clc\n\
                        sec\n\
                        cli\n\
                        sei\n\
                        cld\n\
                        sed\n\
                        clv\n";

    let mut assembler = Assembler::new(AssemblerConfig {
        opcodes: vec![
            Opcode {
                mnemonic: "nop".to_string(),
                binary: 0x00,
                args: vec![],
            },
            Opcode {
                mnemonic: "hlt".to_string(),
                binary: 0x01,
                args: vec![],
            },
            Opcode {
                mnemonic: "clc".to_string(),
                binary: 0x02,
                args: vec![],
            },
            Opcode {
                mnemonic: "sec".to_string(),
                binary: 0x03,
                args: vec![],
            },
            Opcode {
                mnemonic: "cli".to_string(),
                binary: 0x04,
                args: vec![],
            },
            Opcode {
                mnemonic: "sei".to_string(),
                binary: 0x05,
                args: vec![],
            },
            Opcode {
                mnemonic: "cld".to_string(),
                binary: 0x06,
                args: vec![],
            },
            Opcode {
                mnemonic: "sed".to_string(),
                binary: 0x07,
                args: vec![],
            },
            Opcode {
                mnemonic: "clv".to_string(),
                binary: 0x08,
                args: vec![],
            },
        ],
    });

    let result = assembler.assemble(source).unwrap();

    assert_eq!(
        result,
        vec![
            0x00, // nop
            0x01, // hlt
            0x02, // clc
            0x03, // sec
            0x04, // cli
            0x05, // sei
            0x06, // cld
            0x07, // sed
            0x08, // clv
        ]
    );
}

/// Test opcodes with 1 argument, both immediate and indirect.
#[test]
fn test_opcode_1_arg() {
    let source = "lda #0xF1\n\
                        ldb 0xF2\n";

    let mut assembler = Assembler::new(AssemblerConfig {
        opcodes: vec![
            Opcode {
                mnemonic: "lda".to_string(),
                binary: 0x00,
                args: vec![OpcodeArg::Immediate],
            },
            Opcode {
                mnemonic: "ldb".to_string(),
                binary: 0x01,
                args: vec![OpcodeArg::Indirect],
            },
        ],
    });

    let result = assembler.assemble(source).unwrap();

    assert_eq!(result, vec![0x00, 0xF1, 0x01, 0xF2]);
}

/// Test opcodes with more than 1 argument, both immediate and indirect, and mixed.
#[test]
fn assemble_opcodes_many_args() {
    let source = "add #0xF1 0xF2\n\
                        sub 0xF3 #0xF4\n\
                        foo #0x01 #0x83 0x12 0x34";

    let mut assembler = Assembler::new(AssemblerConfig {
        opcodes: vec![
            Opcode {
                mnemonic: "add".to_string(),
                binary: 0x00,
                args: vec![OpcodeArg::Immediate, OpcodeArg::Indirect],
            },
            Opcode {
                mnemonic: "sub".to_string(),
                binary: 0x01,
                args: vec![OpcodeArg::Indirect, OpcodeArg::Immediate],
            },
            Opcode {
                mnemonic: "foo".to_string(),
                binary: 0x02,
                args: vec![
                    OpcodeArg::Immediate,
                    OpcodeArg::Immediate,
                    OpcodeArg::Indirect,
                    OpcodeArg::Indirect,
                ],
            },
        ],
    });

    let result = assembler.assemble(source).unwrap();

    assert_eq!(
        result,
        vec![0x00, 0xF1, 0xF2, 0x01, 0xF3, 0xF4, 0x02, 0x01, 0x83, 0x12, 0x34]
    );
}

/// Test the assembling of labels in the source code, and as arguments to opcodes.
#[test]
fn assemble_labels_args() {
    let source = ".org 0x04\n\
                        TEST_LABEL: jmp TEST_LABEL";

    let mut assembler = Assembler::new(AssemblerConfig {
        opcodes: vec![Opcode {
            mnemonic: "jmp".to_string(),
            binary: 0xF1,
            args: vec![OpcodeArg::Immediate, OpcodeArg::Immediate],
        }],
    });

    let result = assembler.assemble(source).unwrap();

    assert_eq!(result, vec![0x00, 0x00, 0x00, 0x00, 0xF1, 0x04, 0x00,]);
}
