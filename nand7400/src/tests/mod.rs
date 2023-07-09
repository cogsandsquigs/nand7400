#![cfg(test)]

use super::*;

const CONFIG_STR: &str = include_str!("assembly.conf.json");

/// Gets the assembler config for the tests.
fn get_assembler() -> Assembler {
    // The config string is a JSON string that contains the configuration for the assembler.
    let config: AssemblerConfig =
        serde_json::from_str(CONFIG_STR).expect("The config string is invalid JSON!");

    // The assembler is created with the configuration.
    Assembler::new(config)
}

/// Test if we can assemble a basic program.
#[test]
fn test_basic_assembly() {
    let mut assembler = get_assembler();

    let file = include_str!("programs/simple_basic.asm");

    let result = assembler.assemble(file);

    assert_eq!(
        result.unwrap(),
        vec![0x00, 0x01, 0xCA, 0x04, 0x00, 0x07, 0x00, 0x03, 0x01, 0x02, 0x03, 0xFF]
    );
}

/// Test if we can parse comments correctly.
#[test]
fn test_parse_comments() {
    let mut assembler = get_assembler();

    let file = include_str!("programs/with_comments.asm");

    let result = assembler.assemble(file);

    assert_eq!(
        result.unwrap(),
        vec![0x00, 0x01, 0xCA, 0x04, 0x00, 0x07, 0x00, 0x03, 0x01, 0x02, 0x03, 0xFF]
    );
}

/// Test if we can detect invalid argument counts for instructions.
#[test]
fn test_invalid_argument_count() {
    let mut assembler = get_assembler();

    let file = include_str!("programs/invalid_args.asm");

    let result = assembler.assemble(file);

    dbg!(&result);

    assert!(result.is_err());

    let error = &result.unwrap_err()[0];

    assert_eq!(
        error,
        &AssemblerError::WrongNumArgs {
            mnemonic: "add".to_string(),
            expected: 3,
            given: 2,
            opcode_span: (118, 121).into(),
            args_span: (122, 131).into(),
        }
    );
}
