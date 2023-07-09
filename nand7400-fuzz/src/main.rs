mod config;

use afl::fuzz;
use arbitrary::{Arbitrary, Unstructured};
use config::test_config;
use nand7400::Assembler;
use std::str;

/// This sets up fuzzing for the `nand7400` crate.
fn main() {
    fuzz(true, |source: &[u8]| {
        let Ok(source_str) =<&str>::arbitrary(&mut Unstructured::new(source)) else {
            // if there's an error parsing the source, we don't care. We should just exit.
            return;
        };

        let config = test_config();

        let mut assembler = Assembler::new(config);

        let _ = assembler.assemble(source_str);
    });
}
