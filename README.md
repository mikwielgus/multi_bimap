# multi_bimap

Many-to-many bidirectional map in Rust.

This data structure is known under many other names: *bidirectional multimap*,
**bi-multimap*, *multi-bimap*, or sometimes even just *bimap*; in set theory,
it's simply called a *relation*; in graph theory, it's the same as a *bipartite
graph*.

## Usage

### Adding dependency

First, add `multi_bimap` as a dependency to your `Cargo.toml`:

```toml
[dependencies]
multi_bimap = "0.1.2"
```

### Example

A `MultiBimap` keeps two antiparallel maps in sync. Each side may map a key to
multiple values. You can look up associations in either direction.

```rust
use multi_bimap::MultiBimap;
use std::collections::{HashMap, HashSet};

let mut m: MultiBimap<HashMap<&str, HashSet<i32>>, HashMap<i32, HashSet<&str>>> = MultiBimap::new();

m.insert("a", 1);
m.insert("a", 2);
m.insert("b", 1);

// Look up all right values for a left key.
let mut a_rights: Vec<_> = m.left_to_right().get(&"a").unwrap().iter().copied().collect();
a_rights.sort();
assert_eq!(a_rights, [1, 2]);

// Look up all left values for a right key.
let mut one_lefts: Vec<_> = m.right_to_left().get(&1).unwrap().iter().copied().collect();
one_lefts.sort();
assert_eq!(one_lefts, ["a", "b"]);

// Remove one association; empty keys are dropped automatically.
assert_eq!(m.remove(&"a", &1), Some(("a", 1)));
```
