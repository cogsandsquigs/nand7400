#![cfg(test)]

use miette::private::kind::TraitKind;

use super::*;

fn test_config() -> AssemblerConfig {
    AssemblerConfig {
        opcodes: vec![
            Opcode {
                mnemonic: "NOP".into(),
                num_args: 0,
                binary: 0x00,
            },
            Opcode {
                mnemonic: "LDA".into(),
                num_args: 1,
                binary: 0x01,
            },
            Opcode {
                mnemonic: "ADD".into(),
                num_args: 3,
                binary: 0x02,
            },
            Opcode {
                mnemonic: "JMP".into(),
                num_args: 2,
                binary: 0x03,
            },
            Opcode {
                mnemonic: "HLT".into(),
                num_args: 0,
                binary: 0xFF,
            },
        ],
    }
}

#[test]
fn test_basic_assembly() -> miette::Result<()> {
    let mut assembler = Assembler::new(test_config());

    let file = include_str!("programs/test.asm");

    let result = assembler.assemble(file);

    dbg!(&result);

    if let Err(err) = result {
        return Err(err[0].clone().into());
    }

    assert_eq!(
        result.unwrap(),
        vec![0x00, 0x01, 0xCA, 0x03, 0x00, 0x05, 0x00, 0x02, 0x01, 0x02, 0x03, 0xFF]
    );

    Ok(())
}
