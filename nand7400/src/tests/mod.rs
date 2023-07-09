#![cfg(test)]

use super::*;
use miette::Result;

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
fn test_basic_assembly() -> Result<()> {
    let mut assembler = get_assembler();

    let file = include_str!("programs/simple.asm");

    let result = assembler.assemble(file);

    dbg!(&result);

    if let Err(err) = result {
        return Err(err[0].clone().with_source_code(file.to_string()));
    }

    assert_eq!(
        result.unwrap(),
        vec![0x00, 0x01, 0xCA, 0x03, 0x00, 0x07, 0x00, 0x02, 0x01, 0x02, 0x03, 0xFF]
    );

    Ok(())
}

/// Test if we can parse comments correctly.
#[test]
fn test_parse_comments() -> Result<()> {
    let mut assembler = get_assembler();

    let file = include_str!("programs/comments.asm");

    let result = assembler.assemble(file);

    dbg!(&result);

    if let Err(err) = result {
        return Err(err[0].clone().with_source_code(file.to_string()));
    }

    assert_eq!(
        result.unwrap(),
        vec![0x00, 0x01, 0xCA, 0x03, 0x00, 0x07, 0x00, 0x02, 0x01, 0x02, 0x03, 0xFF]
    );

    Ok(())
}
