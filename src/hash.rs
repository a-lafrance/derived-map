use std::collections::HashMap;
use std::hash::Hash;

pub struct DerivedHashMap<'a, K, V, F>
where
	K: Hash + Eq,
	F: Fn(&V) -> &'a K,
{
	derivation: F,
	mapping: HashMap<&'a K, V>,
}

impl<'a, K, V, F> DerivedHashMap<'a, K, V, F>
where
    K: Hash + Eq,
	F: Fn(&V) -> &'a K,
{
	pub fn new(derivation: F) -> Self {
		DerivedHashMap { derivation, mapping: HashMap::new() }
	}

	pub fn get(&'a self, key: &'a K) -> Option<&'a V> {
		// hash key, lookup in map
		self.mapping.get(&key)
	}

	pub fn insert(&'a mut self, val: V) {
		// apply derivation to val
		// insert into map based on derived key
		let key = (self.derivation)(&val);
		self.mapping.insert(key, val);
	}

    pub fn remove(&'a mut self, key: &'a K) -> Option<V> {
        self.mapping.remove(key)
    }
}
