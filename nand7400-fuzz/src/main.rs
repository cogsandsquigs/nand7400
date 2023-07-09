use afl::fuzz;
use arbitrary::{Arbitrary, Unstructured};
use nand7400::{config::AssemblerConfig, Assembler};
use std::str;

const CONFIG_STR: &str = include_str!("assembly.conf.json");

/// This sets up fuzzing for the `nand7400` crate.
fn main() {
    // The config string is a JSON string that contains the configuration for the assembler.
    let config: AssemblerConfig =
        serde_json::from_str(CONFIG_STR).expect("The config string is invalid JSON!");

    // The assembler is created with the configuration.
    let mut assembler = Assembler::new(config);

    fuzz(true, |source: &[u8]| {
        let Ok(source_str) =<&str>::arbitrary(&mut Unstructured::new(source)) else {
            // if there's an error parsing the source, we don't care. We should just exit.
            return;
        };

        let _ = assembler.assemble(source_str);
    });
}
