#![cfg(test)]

use super::*;

/// Test the `.byte` and `.org` keywords.
#[test]
fn test_keywords() {
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
