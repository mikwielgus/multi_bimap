<!--
SPDX-FileCopyrightText: 2025 multi_bimap contributors

SPDX-License-Identifier: MIT OR Apache-2.0
-->

[![Repository](https://img.shields.io/badge/repository-GitHub-0FBF3E)](https://github.com/mikwielgus/multi_bimap)
[![Docs](https://docs.rs/multi_bimap/badge.svg)](https://docs.rs/multi_bimap/)
[![Crates.io](https://img.shields.io/crates/v/multi_bimap.svg)](https://crates.io/crates/multi_bimap)
[![MIT OR Apache 2.0](https://img.shields.io/crates/l/multi_bimap.svg)](#licence)

# multi_bimap

Many-to-many bidirectional map in Rust.

This crate provides a `MultiBimap` struct, a bidirectional multimap that is
implemented as two antiparallel multimaps that are kept in sync.

This structure is also known under many other names: *bi-multimap*,
*multi-bimap*, or sometimes even just *bimap*; in set theory, it's simply called
a *relation*; in graph theory, it's the same as a *bipartite graph*.

## Usage

### Adding dependency

First, add `multi_bimap` as a dependency to your `Cargo.toml`:

```toml
[dependencies]
multi_bimap = "0.2.0"
```

### Example

A `MultiBimap` keeps two antiparallel multimaps in sync; each side may map a key
to multiple values. You can look up associations in either direction.

```rust
use multi_bimap::MultiBimap;
use std::collections::{HashMap, HashSet};

let mut m: MultiBimap<HashMap<&str, HashSet<i32>>, HashMap<i32, HashSet<&str>>> = MultiBimap::new();

m.insert("a", 1);
m.insert("a", 2);
m.insert("b", 1);

// Look up all right values for a left key.
assert_eq!(m.get_by_left("a"), Some(&HashSet::from([1, 2])));

// Look up all left values for a right key.
assert_eq!(m.get_by_right(&1), Some(&HashSet::from(["a", "b"])));

// Remove one association; empty keys are dropped automatically.
assert_eq!(m.remove(&"a", &1), Some(("a", 1)));
```
