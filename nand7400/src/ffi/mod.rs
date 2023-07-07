use crate::{config::AssemblerConfig, errors::AssemblerError, Assembler};
use std::sync::Mutex;

/// The FFI-safe version of the assembler. Called `AssemblerFfi` to avoid name conflicts
/// with `Assembler` from the crate.
pub struct AssemblerFfi {
    inner: Mutex<Assembler>,
}

/// Public API for the assembler.
impl AssemblerFfi {
    /// Create a new assembler with the given configuration.
    pub fn new(config: AssemblerConfig) -> Self {
        Self {
            inner: Mutex::new(Assembler::new(config)),
        }
    }

    /// Assembles the given assembly code into binary.
    /// TODO: Actually call the `assemble()` method.
    pub fn assemble(&self, _source: String) -> Result<Vec<u8>, AssemblerError> {
        // self.inner
        //     .lock()
        //     .as_mut()
        //     .expect("This Mutex should never panic!") // TODO: Get rid of the `expect()` call.
        //     .assemble(source)
        Ok(vec![0xF, 0x0, 0x0, 0xD, 0xB, 0xA, 0xB, 0xE])
    }

    /// Replaces the configuration of the assembler with the given one.
    pub fn set_config(&self, config: AssemblerConfig) {
        self.inner
            .lock()
            .as_mut()
            .expect("This Mutex should never panic!") // TODO: Get rid of the `expect()` call.
            .set_config(config);
    }
}
