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

You can arbitrarily choose what should be the types of the constituent
multimaps: for example, `HashMap<&str, HashSet<&str>>`, `BTreeMap<i64,
Vec<&str>>`, `indexmap::IndexMap<&str, Box<(i64, i64)>` are all valid options.

Even better, these multimaps can have mixed types (e.g. `HashMap` of `HashSet`s
pointing rightwards, `BTreeMap` of `Box`es pointing leftwards), so you can also
have one-to-many or many-to-one bimaps as well (one-to-one bimap is obviously
also an option). This is possible because `MultiBimap` uses traits from
[`maplike`](https://github.com/mikwielgus/maplike), a Rust crate which allows to
have a generic interface over a large number of containers.

This bidirectional multimap relation that `MultiBimap` models is also known
under many other names: *bi-multimap*, *multi-bimap*, or sometimes even just
*bimap*; in set theory, it's simply called a *relation*; in graph theory, it's
the same as a *bipartite graph*.

This crate has no `unsafe` code and is compatible with `serde`
and `undoredo` libraries. It is also `no_std`-compatible, though
[`alloc`](https://doc.rust-lang.org/alloc/) is still required. There is no
mandatory third-party dependencies, but there is a first-party dependency on
[`maplike`](https://github.com/mikwielgus/maplike), a library also developed by
this library's authors.

## Usage

### Adding dependency

First, add `multi_bimap` as a dependency to your `Cargo.toml`:

```toml
[dependencies]
multi_bimap = "0.3.1"
```

### Examples

#### Many-to-many bidirectional map

In academic publishing, the relation between authors and academic papers is
many-to-many; it is a bipartite graph: each author may have many papers, and
each paper may have many authors. A `MultiBimap` can fully represent that:

```rust
use multi_bimap::HashMultiBimap;
use std::collections::{HashMap, HashSet};

let mut authorship: HashMultiBimap<&str, &str> = HashMultiBimap::new();

// `HashMultiBimap` is an alias for a multi-bimap made of two antiparallel hash
// maps with hash sets as values. Without alias the above line would be this:

/*let mut authorship: MultiBimap<
    HashMap<&str, HashSet<&str>>,
    HashMap<&str, HashSet<&str>>,
> = MultiBimap::new();*/

authorship.insert("Alan Turing", "On Computable Numbers");
authorship.insert("Alan Turing", "Computing Machinery and Intelligence");
authorship.insert("Ada Lovelace", "Notes on the Analytical Engine");
authorship.insert("Charles Babbage", "Notes on the Analytical Engine");

// Papers by one author.
assert_eq!(
    authorship.get_by_left("Alan Turing"),
    Some(&HashSet::from([
        "On Computable Numbers",
        "Computing Machinery and Intelligence",
    ])),
);

// Authors of one paper.
assert_eq!(
    authorship.get_by_right("Notes on the Analytical Engine"),
    Some(&HashSet::from(["Ada Lovelace", "Charles Babbage"])),
);

// Remove one author-paper association. Empty keys will disappear from both
// sides.
assert_eq!(
    authorship.remove(&"Charles Babbage", &"Notes on the Analytical Engine"),
    Some(("Charles Babbage", "Notes on the Analytical Engine")),
);
assert_eq!(
    authorship.get_by_right("Notes on the Analytical Engine"),
    Some(&HashSet::from(["Ada Lovelace"])),
);
assert_eq!(authorship.get_by_left("Charles Babbage"), None);
```

## Documentation

See the [documentation](https://docs.rs/multi_bimap/latest/multi_bimap) for more information
on `multi_bimap`'s usage.

## Packaging

`multi_bimap` is published as a [crate](https://crates.io/crates/multi_bimap) on the
[Crates.io](https://crates.io/) registry.

## Contributing

We welcome issues, pull requests and any other contributions from anyone to our
[repository](https://github.com/mikwielgus/multi_bimap) on GitHub.

## Licence

### Outbound licence

`multi_bimap` is dual-licensed as under

- [MIT license](./LICENSES/MIT.txt), or
- [Apache License, Version 2.0](./LICENSES/Apache-2.0.txt),

at your option.

### Inbound licence

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this work by you will be dual-licensed as described above,
without any additional terms or conditions.
