#![cfg(test)]

use super::*;

#[test]
fn test_assembling() {
    let mut assembler = Assembler::new(AssemblerConfig { opcodes: vec![] });

    let file = include_str!("programs/test.asm");

    let result = assembler.assemble(file);
}
