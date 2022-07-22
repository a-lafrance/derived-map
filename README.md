# derived-map
Maps with derived keys

Somewhat often I find myself wishing for a "derived map", where it's a normal maps where the keys are derived from the values. A common use case is where I want to map some kind of structure by its identifier, which I already know to be unique (could be its name, its numeric ID, who knows). It's not quite satisfactory to clone or copy it as the key, because that introduces an invariant between keys and values that isn't captured concretely anywhere in the map.

Enter the derived map, where keys are derived from values via some transformation. In my common use case, it would be a simple closure retrieving the identifier from the data structure. This captures the relationship between key and value through the derivation function.

One key property of derived maps: they don't support mutation of values. The reason for this should be pretty clear: if you allow mutable access to a value, you're also allowing mutable access to its key (kind of implicitly), so the key (and thus the hash) could be "poisoned" through mutation. Depending on the internals of the implementation this _might_ not be a problem with some clever tricks, but right now it is. So basically only use derived maps when you want a link between a value and its derived key without needing to mutate the value.

For the time being this is a highly experimental piece of code, mostly assembled based on a one night prototyping spree. In due time I'll dress it up in a nice crate and stick it on `crates.io`.
