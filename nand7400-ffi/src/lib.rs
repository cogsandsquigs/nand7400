use config::Opcode;
use miette::Diagnostic;
pub use nand7400::config;
use nand7400::errors::AssemblerError as RustAssemblerError;
use nand7400::{config::AssemblerConfig, Assembler as RustAssembler};
use snafu::Snafu;
use std::sync::Mutex;

// Need to include this so that UniFFI scaffolding is generated.
uniffi::include_scaffolding!("ffi");

/// The FFI-safe version of the assembler from the `nand7400` crate.
pub struct Assembler {
    inner: Mutex<RustAssembler>,
}

/// Public API for the assembler.
impl Assembler {
    /// Create a new assembler with the given configuration.
    pub fn new(config: AssemblerConfig) -> Self {
        Self {
            inner: Mutex::new(RustAssembler::new(config)),
        }
    }

    /// Replaces the configuration of the assembler with the given one.
    pub fn set_config(&self, config: AssemblerConfig) {
        self.inner
            .lock()
            .as_mut()
            .expect("This Mutex should never panic!") // TODO: Get rid of the `expect()` call.
            .set_config(config);
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
}

/// The assembler error type.
#[derive(Clone, Debug, PartialEq, Eq, Snafu, Diagnostic)]
pub enum AssemblerError {
    /// Just a wrapper around an error.
    Error {
        #[diagnostic_source]
        source: RustAssemblerError,
    },
}
