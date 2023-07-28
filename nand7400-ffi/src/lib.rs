pub use nand7400::{
    ast::Statement,
    config::{AssemblerConfig, Opcode},
    errors::AssemblerError,
    position::Position,
};

use nand7400::{ast::Ast as RustAst, Assembler as RustAssembler};
use std::sync::Mutex;

// Need to include this so that UniFFI scaffolding is generated.
uniffi::include_scaffolding!("ffi");

/// The FFI-safe version of the assembler from the `nand7400` crate.
pub struct Assembler {
    /// This is the inner assembler that is run in a mutex. The mutex is needed because UniFFI requires that all
    /// bound functions are Send, Sync, and have no mutable reference functions. The mutex is used to ensure that
    /// only one thread can access the inner assembler at a time, and also act like a RefCell so that the inner
    /// assembler can be mutated without using a mutable reference function.
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
            .expect("An internal Mutex was poisoned! Some thread must have panicked while holding onto this Mutex!") 
            .set_config(config);
    }

    /// Assembles the given assembly code into binary.
    pub fn assemble(&self, source: &str) -> Result<Vec<u8>, AssemblerErrorCollection> {
        self.inner
            .lock()
            .as_mut()
            .expect("An internal Mutex was poisoned! Some thread must have panicked while holding onto this Mutex!")
            .assemble(source)
            .map_err(|err| AssemblerErrorCollection::Errors {
                errors: err,
            })
    }

    /// Assembles the given assembly code into binary and associated AST.
    pub fn assemble_with_ast(
        &self,
        source: &str,
    ) -> Result<(Vec<u8>, Ast), AssemblerErrorCollection> {
        self.inner
            .lock()
            .as_mut()
            .expect("An internal Mutex was poisoned! Some thread must have panicked while holding onto this Mutex!")
            .assemble_with_ast(source)
            .map(|(binary, ast)| (binary, Ast { ast }))
            .map_err(|err| AssemblerErrorCollection::Errors {
                errors: err,
            })
    }
}

/// A wrapper around the Ast type that is FFI-safe.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Ast {
    /// The core ast type.
    ast: RustAst,
}

impl Ast {
    /// Create a new binary  with no instructions.
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            ast: RustAst::new(),
        }
    }

    /// Gets the length of the binary, in bytes.
    pub fn len(&self) -> u16 {
        self.ast.len()
    }

    /// Check if the binary is empty.
    pub fn is_empty(&self) -> bool {
        self.ast.is_empty()
    }

    /// Returns an array of all the binary instructions in the binary.
    pub fn statements(&self) -> Vec<Statement> {
        self.ast.statements().to_vec()
    }
}

/// A wrapper around many assembler errors.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
#[error("Multiple assembler errors were found.")]
pub enum AssemblerErrorCollection {
    /// The errors that were collected.
    Errors { errors: Vec<AssemblerError> },
}
