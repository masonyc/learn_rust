#![deny(missing_docs)]

//! talent plan toy kv store

use std::collections::HashMap;

/// KvStore that stores in memory
pub struct KvStore {
    /// hashmap store
    in_memory_store: HashMap<String, String>,
}

impl Default for KvStore {
    fn default() -> Self {
        Self::new()
    }
}

/// Provide simple get/set/remove for in memory store
/// # Examples
impl KvStore {
    /// Init store with in memory HashMap
    ///
    /// # Examples
    ///
    /// ```
    /// use kvs::KvStore;
    ///
    /// let kv_store = KvStore::new();
    /// ```
    pub fn new() -> KvStore {
        KvStore {
            in_memory_store: HashMap::new(),
        }
    }

    /// set item in in_memory_store
    ///
    /// # Examples
    /// ```
    /// use kvs::KvStore;
    ///
    /// let mut kv_store = KvStore::new();
    /// kv_store.set("key1".to_owned(), "value1".to_owned());
    /// assert_eq!(kv_store.get("key1".to_owned()), Some("value1".to_owned()));
    pub fn set(&mut self, key: String, value: String) {
        self.in_memory_store.insert(key, value);
        println!("{:#?}", self.in_memory_store)
    }

    ///
    pub fn remove(&mut self, key: String) {
        self.in_memory_store.remove(&key);
    }

    ///
    pub fn get(&self, key: String) -> Option<String> {
        self.in_memory_store.get(&key).map(|s| s.to_string())
    }
}
