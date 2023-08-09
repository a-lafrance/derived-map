# derived-map
Maps with derived keys

I've sometimes found myself needing something like a map where the keys are derived from the values. The use case for this is almost always that I want to map from some kind of unique identifier to some data, but the identifier must be owned within the data itself. Cloning or copying it as the key is technically possible but not really satisfying because it can often be somewhat costly and because from a design perspective it introduces a magic invariant between keys and values that isn't captured or enforced anywhere in the map.

Enter the derived map, which does exactly that: it's a map where keys may be derived from values via some transformation, and queries can be made directly using key values. In my motivating use case, this transformation would be a super simple closure grabbing the identifier from the data structure. In my opinion this provides a cleaner abstraction by wrapping around the internals of enforcing synchronization between keys and values.

Right now all I've got is a super flimsy wrapper around Rust's map structures as a proof of concept, but what I'm imagining in an ideal world is more robust. In the identifiers-as-keys use case specifically, if the identifiers are guaranteed to be unique, it might be possible to make things even cleaner with a perfect hash function. Or, if not a mathematically perfect hash function itself, some kind of scheme to store the values with less overhead because you know there won't be any collisions between keys (maybe implementing it with some form of interning?). Not too sure yet, but might be cool.

One thing I'm not sure about is mutation of values. The current prototype doesn't support it (conceptually to avoid "poisining" the corresponding key through mutation, practically because of lifetime issues), but with a pinch of unsafe code and some clever implementation tricks I'm sure this could be addressed. That said, I'm not sure if that's worth it -- I've found that the cases where this kind of thing is necessary tend to be very insert-then-query, and once the values are inserted they don't really need to be modified. But if that changes, support for mutation is something to explore.

Anyway, I'll _probably_ explore this in detail and get an actual library in shape for `crates.io`, if I can find the time and motivation someday.
