use std::collections::BTreeMap;

pub struct DerivedBTreeMap<'a, K, V, F>
where
	K: Ord,
	F: Fn(&V) -> &'a K,
{
	derivation: F,
	mapping: BTreeMap<&'a K, V>,
}

impl<'a, K, V, F> DerivedBTreeMap<'a, K, V, F>
where
	K: Ord,
	F: Fn(&V) -> &'a K,
{
	pub fn new(derivation: F) -> Self {
		DerivedBTreeMap { derivation, mapping: BTreeMap::new() }
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
}
