use nand7400::config::{AssemblerConfig, Opcode};

/// This is a testing configuration for the assembler.
pub fn test_config() -> AssemblerConfig {
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
