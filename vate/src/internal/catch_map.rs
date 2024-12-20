//! A hashmap with an optional catch value.

use std::{borrow::Borrow, collections::HashMap, hash::Hash};

/// A hashmap with an optional catch value.
pub struct CatchMap<K, V> {
    /// The primary values.
    primary_values: HashMap<K, V>,

    /// The catch value.
    catch_value: Option<V>,
}

impl<K, V> Default for CatchMap<K, V> {
    fn default() -> Self {
        Self {
            primary_values: HashMap::default(),
            catch_value: None,
        }
    }
}

impl<K, V> CatchMap<K, V>
where
    K: Hash + Eq,
{
    /// Insert a primary value.
    ///
    /// Returns the old value if replaced.
    pub fn insert_primary(&mut self, key: K, value: V) -> Option<V> {
        self.primary_values.insert(key, value)
    }

    /// Get a primary value.
    pub fn get_primary<Q>(&self, key: &Q) -> Option<&V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        self.primary_values.get(key)
    }

    /// Get a primary value or the catch value.
    pub fn get_primary_or_catch<Q>(&self, key: &Q) -> Option<&V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        self.get_primary(key).or(self.get_catch())
    }

    /// Get a primary value or insert the default.
    pub fn get_primary_or_insert_default(&mut self, key: K) -> &mut V
    where
        V: Default,
    {
        self.primary_values.entry(key).or_default()
    }

    /// Set the catch value.
    ///
    /// Returns the old value if replaced.
    pub fn set_catch(&mut self, catch_value: V) -> Option<V> {
        self.catch_value.replace(catch_value)
    }

    /// Get the catch value.
    pub fn get_catch(&self) -> Option<&V> {
        self.catch_value.as_ref()
    }

    /// Get the catch value or insert the default.
    pub fn get_catch_or_insert_default(&mut self) -> &mut V
    where
        V: Default,
    {
        self.catch_value.get_or_insert_with(V::default)
    }
}
