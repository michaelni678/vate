//! Core types and traits.

use std::{borrow::Borrow, collections::HashMap, fmt, hash::Hash};

// Re-export Validate derive macro.
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
        interpreter_data: &D,
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
        interpreter_data: &D,
        report: &mut R,
    ) -> Result<ControlFlow, E>
    where
        R: Report;
}

/// The control flow for validators.
#[must_use]
pub enum ControlFlow {
    Continue,
    Exit,
}

/// Contains information regarding validations.
#[derive(Clone)]
pub struct Invalid<'a> {
    /// The type ident.
    pub type_ident: TypeIdent,

    /// The field ident.
    pub field_ident: FieldIdent,

    /// The validator idents.
    pub validator_idents: Vec<ValidatorIdent>,

    /// All the details for the validators.
    pub all_validator_details: Vec<Details<'a>>,
}

impl<'a> Invalid<'a> {
    /// Add a validation.
    pub fn add_validation(
        mut self,
        validator_ident: ValidatorIdent,
        validator_details: Details<'a>,
    ) -> Self {
        self.validator_idents.push(validator_ident);
        self.all_validator_details.push(validator_details);
        self
    }
}

/// A type ident.
#[derive(Copy, Clone, Hash, PartialEq, Eq)]
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

impl fmt::Display for TypeIdent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Struct(ident) => write!(f, "{ident}"),
            Self::Enum(ident, variant) => write!(f, "{ident}::{variant}"),
        }
    }
}

/// A field ident.
#[derive(Copy, Clone, Hash, PartialEq, Eq)]
pub enum FieldIdent {
    /// A named field ident.
    ///
    /// For example, `a` in `struct X { a: i32 }`.
    Named(&'static str),

    /// An unnamed field ident.
    ///
    /// For example, `0` in `struct X(i32)`.
    Unnamed(usize),
}

impl fmt::Display for FieldIdent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Named(ident) => write!(f, "{ident}"),
            Self::Unnamed(ident) => write!(f, "{ident}"),
        }
    }
}

/// A validator ident.
#[derive(Copy, Clone, Hash, PartialEq, Eq)]
pub struct ValidatorIdent {
    /// The name of the validator.
    pub name: &'static str,

    /// The variant of the validator.
    pub variant: u8,
}

impl fmt::Display for ValidatorIdent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}#{}", self.name, self.variant)
    }
}

/// A validator's details.
#[derive(Default, Clone)]
pub struct Details<'a>(Vec<&'a dyn ToString>);

impl<'a> Details<'a> {
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

/// Defines a report that holds invalid validations.
pub trait Report {
    /// Check if the report is valid.
    fn is_valid(&self) -> bool;

    /// Check if the report is invalid.
    fn is_invalid(&self) -> bool {
        !self.is_valid()
    }

    /// Split the current report.
    fn split(&mut self) -> Self;

    /// Merge a report with another.
    fn merge(&mut self, other: Self);

    /// Add an invalid to the report.
    fn add_invalid<D>(
        &mut self,
        invalid: Invalid,
        interpreter: &Interpreter<D>,
        interpreter_data: &D,
    ) -> ControlFlow;
}

/// Interprets invalid validations into messages.
pub struct Interpreter<D> {
    /// Override interpreter functions.
    #[allow(clippy::type_complexity)]
    override_functions: CatchMap<
        TypeIdent,
        CatchMap<FieldIdent, CatchMap<Box<[ValidatorIdent]>, InterpreterFunction<D>>>,
    >,

    /// Normal interpreter functions mapped by validator idents.
    normal_functions: HashMap<Box<[ValidatorIdent]>, InterpreterFunction<D>>,

    /// Fallback interpreter function.
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
        validator_idents: Option<Vec<ValidatorIdent>>,
        function: impl Into<InterpreterFunction<D>>,
    ) {
        let a = match type_ident {
            Some(type_ident) => self.override_functions.get_or_insert_default(type_ident),
            None => self.override_functions.get_catch_value_or_insert_default(),
        };

        let b = match field_ident {
            Some(field_ident) => a.get_or_insert_default(field_ident),
            None => a.get_catch_value_or_insert_default(),
        };

        match validator_idents {
            Some(validator_idents) => {
                b.insert(validator_idents.into_boxed_slice(), function.into())
            }
            None => b.set_catch_value(function.into()),
        };
    }

    /// Get an override function.
    pub fn get_override_function(
        &self,
        type_ident: &TypeIdent,
        field_ident: &FieldIdent,
        validator_idents: &[ValidatorIdent],
    ) -> Option<&InterpreterFunction<D>> {
        self.override_functions
            .get_or_catch_value(type_ident)?
            .get_or_catch_value(field_ident)?
            .get_or_catch_value(validator_idents)
    }

    /// Set a normal function.
    pub fn set_normal_function(
        &mut self,
        validator_idents: Vec<ValidatorIdent>,
        function: impl Into<InterpreterFunction<D>>,
    ) {
        self.normal_functions
            .insert(validator_idents.into_boxed_slice(), function.into());
    }

    /// Get a normal function.
    pub fn get_normal_function(
        &self,
        validator_idents: &[ValidatorIdent],
    ) -> Option<&InterpreterFunction<D>> {
        self.normal_functions.get(validator_idents)
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
        validator_idents: &[ValidatorIdent],
    ) -> &InterpreterFunction<D> {
        let override_function =
            self.get_override_function(type_ident, field_ident, validator_idents);

        match override_function {
            Some(f) => f,
            None => self
                .get_normal_function(validator_idents)
                .unwrap_or(self.get_fallback_function()),
        }
    }

    /// Interpret an invalid validation.
    pub fn interpret(&self, data: &D, invalid: &Invalid) -> Option<String> {
        let function = self.get_function(
            &invalid.type_ident,
            &invalid.field_ident,
            &invalid.validator_idents,
        );

        (function.0)(data, invalid)
    }
}

/// Function for interpreting invalid validations into messages.
pub struct InterpreterFunction<D>(
    #[allow(clippy::type_complexity)] Box<dyn Fn(&D, &Invalid) -> Option<String> + Send + Sync>,
);

impl<D> Default for InterpreterFunction<D> {
    fn default() -> Self {
        Self(Box::new(|_data, invalid| {
            Some(format!("{} is invalid", invalid.field_ident))
        }))
    }
}

impl<D, F> From<F> for InterpreterFunction<D>
where
    F: Fn(&D, &Invalid) -> Option<String> + Send + Sync + 'static,
{
    fn from(function: F) -> Self {
        Self(Box::new(function))
    }
}

/// A hashmap with an optional catch value.
struct CatchMap<K, V> {
    /// The inner hashmap.
    inner: HashMap<K, V>,

    /// The catch value.
    catch_value: Option<V>,
}

impl<K, V> Default for CatchMap<K, V> {
    fn default() -> Self {
        Self {
            inner: HashMap::default(),
            catch_value: None,
        }
    }
}

impl<K, V> CatchMap<K, V>
where
    K: Hash + Eq,
{
    /// Get the value associated with a key, or the catch value.
    fn get_or_catch_value<Q>(&self, key: &Q) -> Option<&V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        self.inner.get(key).or(self.get_catch_value())
    }

    fn insert(&mut self, key: K, value: V) {
        self.inner.insert(key, value);
    }

    /// Get a value or the default.
    fn get_or_insert_default(&mut self, key: K) -> &mut V
    where
        V: Default,
    {
        self.inner.entry(key).or_default()
    }

    /// Set the catch value.
    fn set_catch_value(&mut self, catch_value: V) {
        self.catch_value.replace(catch_value);
    }

    /// Get the catch value.
    fn get_catch_value(&self) -> Option<&V> {
        self.catch_value.as_ref()
    }

    /// Get the catch value or insert default.
    fn get_catch_value_or_insert_default(&mut self) -> &mut V
    where
        V: Default,
    {
        self.catch_value.get_or_insert_with(V::default)
    }
}
