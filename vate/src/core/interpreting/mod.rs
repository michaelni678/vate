//! Interpreting types and traits.

pub(super) mod catch_map;
pub mod interpreter;

pub use interpreter::*;

/// Resources necessary for interpreting.
pub struct InterpretingArgs<'a, D> {
    /// Holds the logic for interpreting drafts into messages.
    pub interpreter: &'a Interpreter<D>,

    /// Custom data for interpreting.
    pub data: &'a D,
}
