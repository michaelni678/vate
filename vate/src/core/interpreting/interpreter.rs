use std::collections::HashMap;

use crate::core::{
    interpreting::catch_map::CatchMap, FieldIdent, Invalid, TypeIdent, ValidatorIdent,
};

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
