use nand7400::{
    assembler::{config::AssemblerConfig, Assembler},
    formatter::Formatter,
};

const CONFIG_STR: &str = include_str!("assembly.conf.json");

const ASSEMBLY: &str = include_str!("assembly.asm");

/// Gets the assembler config for the tests.
fn get_assembler() -> Assembler {
    // The config string is a JSON string that contains the configuration for the assembler.
    let config: AssemblerConfig =
        serde_json::from_str(CONFIG_STR).expect("The config string is invalid JSON!");

    // The assembler is created with the configuration.
    Assembler::new(config)
}

fn main() -> miette::Result<()> {
    let mut assembler = get_assembler();

    // The assembly is assembled into a binary.
    let binary = assembler
        .assemble(ASSEMBLY)
        .map_err(|e| e.with_source_code(ASSEMBLY.to_string()))?;

    // The binary is printed to the console.
    println!("{:?}", binary);

    // Print formatted code.
    println!("-----\nFormatted code:");
    println!("{}", Formatter::default().format(ASSEMBLY));

    Ok(())
}
