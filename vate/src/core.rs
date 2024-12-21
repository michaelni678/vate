//! Core types and traits.

use std::collections::HashMap;

use crate::internal::catch_map::CatchMap;

#[cfg(feature = "serialize")]
use serde::Serialize;

// Re-export `Validate` derive macro.
pub use vate_macros::Validate;

/// Allows the implementor to be validated.
pub trait Validate {
    /// Custom context type.
    type Context;

    /// Custom error type.
    type Error;

    /// Validate an instance of the implementor.
    fn validate<D, R>(
        &self,
        context: &Self::Context,
        interpreter: &Interpreter<D>,
        data: &D,
        report: &mut R,
    ) -> Result<ControlFlow, Self::Error>
    where
        R: Report;
}

/// Defines a validator.
pub trait Validator<T, C, E> {
    /// Runs the validator.
    fn run<D, R>(
        self,
        target: T,
        context: &C,
        invalid: Invalid,
        interpreter: &Interpreter<D>,
        data: &D,
        report: &mut R,
    ) -> Result<ControlFlow, E>
    where
        R: Report;
}

/// Contains information regarding validations.
#[derive(Clone)]
pub struct Invalid<'a> {
    /// The type ident.
    pub type_ident: TypeIdent,

    /// The field ident.
    pub field_ident: FieldIdent,

    /// The validation tags.
    pub vtags: Vec<ValidationTag>,

    /// The detailers.
    pub detailers: Vec<Detailer<'a>>,
}

impl<'a> Invalid<'a> {
    /// Push a validation.
    pub fn push_validation(mut self, vtag: ValidationTag, detailer: Detailer<'a>) -> Self {
        self.vtags.push(vtag);
        self.detailers.push(detailer);
        self
    }
}

/// A type ident.
#[derive(Hash, PartialEq, Eq, Copy, Clone)]
#[cfg_attr(feature = "serialize", derive(Serialize))]
pub enum TypeIdent {
    /// A struct's ident.
    ///
    /// For example, `X` in `struct X`.
    Struct(&'static str),

    /// An enum's ident and variant.
    ///
    /// For example, `X` and `Y` in `enum X { Y }`.
    Enum(&'static str, &'static str),
}

/// A field ident.
#[derive(Hash, PartialEq, Eq, Copy, Clone)]
#[cfg_attr(feature = "serialize", derive(Serialize))]
pub enum FieldIdent {
    /// A named field ident.
    ///
    /// For example, field `a` in `struct X { a: i32 }`.
    Named(&'static str),

    /// An unnamed field ident.
    ///
    /// For example, field `0` in `struct X(i32)`.
    Unnamed(usize),
}

/// A validation tag.
pub type ValidationTag = &'static str;

/// Temporarily holds a validator's details.
#[derive(Default, Clone)]
pub struct Detailer<'a>(Vec<&'a dyn ToString>);

impl<'a> Detailer<'a> {
    /// Set a detail at an index.
    pub fn set_detail(mut self, index: usize, detail: &'a dyn ToString) -> Self {
        if index >= self.0.len() {
            // OPTIMIZE: this may resize multiple times depending on order that
            // details are added. Maybe reserve extra capacity or something.
            self.0.resize_with(index + 1, || &"");
        }

        self.0[index] = detail;
        self
    }

    /// Get a detail at an index as a string.
    ///
    /// Panics if out of bounds.
    pub fn get_detail(&self, index: usize) -> String {
        self.0[index].to_string()
    }
}

/// Interpreters invalids into messages.
pub struct Interpreter<D> {
    /// Functions for overriding specific type idents, field idents, etc.
    #[allow(clippy::type_complexity)]
    override_functions: CatchMap<
        TypeIdent,
        CatchMap<FieldIdent, CatchMap<Box<[ValidationTag]>, InterpreterFunction<D>>>,
    >,

    /// Functions mapped by validation tags.
    normal_functions: HashMap<Box<[ValidationTag]>, InterpreterFunction<D>>,

    /// The fallback function.
    fallback_function: InterpreterFunction<D>,
}

impl<D> Default for Interpreter<D> {
    fn default() -> Self {
        Self {
            override_functions: CatchMap::default(),
            normal_functions: HashMap::default(),
            fallback_function: InterpreterFunction::default(),
        }
    }
}

impl<D> Interpreter<D> {
    /// Set an override function.
    pub fn set_override_function(
        &mut self,
        type_ident: Option<TypeIdent>,
        field_ident: Option<FieldIdent>,
        validation_tags: Option<Vec<ValidationTag>>,
        function: impl Into<InterpreterFunction<D>>,
    ) {
        let a = match type_ident {
            Some(type_ident) => self
                .override_functions
                .get_primary_or_insert_default(type_ident),
            None => self.override_functions.get_catch_or_insert_default(),
        };

        let b = match field_ident {
            Some(field_ident) => a.get_primary_or_insert_default(field_ident),
            None => a.get_catch_or_insert_default(),
        };

        match validation_tags {
            Some(validation_tags) => {
                b.insert_primary(validation_tags.into_boxed_slice(), function.into())
            }
            None => b.set_catch(function.into()),
        };
    }

    /// Get an override function.
    pub fn get_override_function(
        &self,
        type_ident: &TypeIdent,
        field_ident: &FieldIdent,
        validation_tags: &[ValidationTag],
    ) -> Option<&InterpreterFunction<D>> {
        self.override_functions
            .get_primary_or_catch(type_ident)?
            .get_primary_or_catch(field_ident)?
            .get_primary_or_catch(validation_tags)
    }

    /// Set a normal function.
    pub fn set_normal_function(
        &mut self,
        validation_tags: Vec<ValidationTag>,
        function: impl Into<InterpreterFunction<D>>,
    ) {
        self.normal_functions
            .insert(validation_tags.into_boxed_slice(), function.into());
    }

    /// Get a normal function.
    pub fn get_normal_function(
        &self,
        validation_tags: &[ValidationTag],
    ) -> Option<&InterpreterFunction<D>> {
        self.normal_functions.get(validation_tags)
    }

    /// Set the fallback function.
    pub fn set_fallback_function(&mut self, function: impl Into<InterpreterFunction<D>>) {
        self.fallback_function = function.into();
    }

    /// Get the fallback function.
    pub fn get_fallback_function(&self) -> &InterpreterFunction<D> {
        &self.fallback_function
    }

    /// Get a function.
    ///
    /// Tries to get functions in the order of:
    /// - Override
    /// - Normal
    /// - Fallback
    pub fn get_function(
        &self,
        type_ident: &TypeIdent,
        field_ident: &FieldIdent,
        validation_tags: &[ValidationTag],
    ) -> &InterpreterFunction<D> {
        let override_function =
            self.get_override_function(type_ident, field_ident, validation_tags);

        match override_function {
            Some(f) => f,
            None => self
                .get_normal_function(validation_tags)
                .unwrap_or(self.get_fallback_function()),
        }
    }

    /// Interpret an invalid validation.
    pub fn interpret(&self, invalid: &Invalid, data: &D) -> Option<String> {
        let function = self.get_function(&invalid.type_ident, &invalid.field_ident, &invalid.vtags);

        (function.inner)(invalid, data)
    }
}

/// A function for interpreting an invalid validation into a message.
pub struct InterpreterFunction<D> {
    #[allow(clippy::type_complexity)]
    pub inner: Box<dyn Fn(&Invalid, &D) -> Option<String> + Send + Sync>,
}

impl<D> Default for InterpreterFunction<D> {
    fn default() -> Self {
        Self {
            inner: Box::new(|_invalid, _data| Some(String::from("invalid"))),
        }
    }
}

impl<D, F> From<F> for InterpreterFunction<D>
where
    F: Fn(&Invalid, &D) -> Option<String> + Send + Sync + 'static,
{
    fn from(function: F) -> Self {
        Self {
            inner: Box::new(function),
        }
    }
}

/// Processes and collects invalids.
pub trait Report {
    /// Get the number of invalids the report has pushed.
    fn num_invalids(&self) -> usize;

    /// Push an invalid to the report.
    fn push_invalid<D>(
        &mut self,
        invalid: Invalid,
        interpreter: &Interpreter<D>,
        data: &D,
    ) -> ControlFlow;
}

/// The control flow for validators.
#[must_use]
pub enum ControlFlow {
    Continue,
    Exit,
}
