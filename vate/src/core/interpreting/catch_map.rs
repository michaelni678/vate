use std::{borrow::Borrow, collections::HashMap, hash::Hash};

/// A hashmap with an optional catch value.
pub struct CatchMap<K, V> {
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
    pub fn get_or_catch_value<Q>(&self, key: &Q) -> Option<&V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        self.inner.get(key).or(self.get_catch_value())
    }

    pub fn insert(&mut self, key: K, value: V) {
        self.inner.insert(key, value);
    }

    /// Get a value or the default.
    pub fn get_or_insert_default(&mut self, key: K) -> &mut V
    where
        V: Default,
    {
        self.inner.entry(key).or_default()
    }

    /// Set the catch value.
    pub fn set_catch_value(&mut self, catch_value: V) {
        self.catch_value.replace(catch_value);
    }

    /// Get the catch value.
    pub fn get_catch_value(&self) -> Option<&V> {
        self.catch_value.as_ref()
    }

    /// Get the catch value or insert default.
    pub fn get_catch_value_or_insert_default(&mut self) -> &mut V
    where
        V: Default,
    {
        self.catch_value.get_or_insert_with(V::default)
    }
}
