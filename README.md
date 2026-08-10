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
multi_bimap = "0.4.0"
```

### Examples

#### Many-to-many bidirectional map

In academic publishing, the relation between authors and academic papers
is many-to-many; it is a bipartite graph: each author may have many papers,
and each paper may have many authors. A `MultiBimap` (here under alias
`HashMultiBimap`) can fully represent that:

```rust
use multi_bimap::HashMultiBimap;
use std::collections::HashSet;

fn main() {
    let mut authorship: HashMultiBimap<&str, &str> = HashMultiBimap::new();

    // `HashMultiBimap` is an alias for a multi-bimap made of two antiparallel hash
    // maps with hash sets as values. Without alias the above line would be this:

    /*let mut authorship: MultiBimap<
        HashMap<&str, HashSet<&str>>,
        HashMap<&str, HashSet<&str>>,
    > = MultiBimap::new();*/

    authorship.insert("Stefan Banach", "Sur les opérations dans les ensembles abstraits");
    authorship.insert("Stefan Banach", "Théorie des opérations linéaires");
    authorship.insert("Stefan Banach", "Sur le principe de la condensation des singularités");
    authorship.insert("Hugo Steinhaus", "Sur le principe de la condensation des singularités");

    // Papers where Stefan Banach is an author.
    assert_eq!(
        authorship.get_by_left("Stefan Banach"),
        Some(&HashSet::from([
            "Sur les opérations dans les ensembles abstraits",
            "Théorie des opérations linéaires",
            "Sur le principe de la condensation des singularités",
        ])),
    );

    // "Sur le principe de la condensation des singularités" has two authors; it was
    // co-authored by Stefan Banach and Hugo Steinhaus.
    assert_eq!(
        authorship.get_by_right("Sur le principe de la condensation des singularités"),
        Some(&HashSet::from(["Stefan Banach", "Hugo Steinhaus"])),
    );

    // As an example, remove one author-paper association, of Hugo Steinhaus with
    // this paper. Empty keys will disappear from both sides.
    assert_eq!(
        authorship.remove(
            &"Hugo Steinhaus",
            &"Sur le principe de la condensation des singularités",
        ),
        Some((
            "Hugo Steinhaus",
            "Sur le principe de la condensation des singularités",
        )),
    );
    assert_eq!(
        authorship.get_by_right("Sur le principe de la condensation des singularités"),
        Some(&HashSet::from(["Stefan Banach"])),
    );
    assert_eq!(authorship.get_by_left("Hugo Steinhaus"), None);

    // Restore Hugo Steinhaus as co-author.
    authorship.insert(
        "Hugo Steinhaus",
        "Sur le principe de la condensation des singularités",
    );
    assert_eq!(
        authorship.get_by_right("Sur le principe de la condensation des singularités"),
        Some(&HashSet::from(["Stefan Banach", "Hugo Steinhaus"])),
    );
    assert_eq!(
        authorship.get_by_left("Hugo Steinhaus"),
        Some(&HashSet::from([
            "Sur le principe de la condensation des singularités",
        ])),
    );
}
```

#### One-to-one bidirectional map

A country and its capital form a one-to-one relation: each country has one
capital, and each capital belongs to one country. `MultiBimap` (here under alias
`HashBimap`) can represent that just as well without any additional logic:

```rust
use maplike::one::One;
use multi_bimap::HashBimap;

fn main() {
    let mut capitals: HashBimap<&str, &str> = HashBimap::new();

    // `HashBimap` is an alias for a one-to-one bimap made of two antiparallel hash
    // maps with `One` as value type. `One` is a special container that can hold
    // only one element that gets displaced upon insert. Without alias the above
    // line would be this:

    /*let mut capitals: MultiBimap<
        HashMap<&str, One<&str>>,
        HashMap<&str, One<&str>>,
    > = MultiBimap::new();*/

    capitals.insert("Poland", "Warsaw");
    capitals.insert("France", "Paris");
    capitals.insert("Lithuania", "Vilnius");

    // Unfortunately, in 1920, Vilnius was annexed by Poland and held until 1939,
    // a regrettable episode in Polish history. During that period Lithuania's capital
    // was Kaunas.
    assert_eq!(
        capitals.insert("Lithuania", "Kaunas"),
        (Some("Vilnius"), None),
    );

    assert_eq!(capitals.get_by_left("Lithuania"), Some(&One::new("Kaunas")));

    // After Lithuania regained Vilnius in 1939, as a side effect of Nazi Germany's
    // and Soviet Union's joint invasion and annexation of Poland, it was restored
    // as Lithuania's capital.
    assert_eq!(
        capitals.insert("Lithuania", "Vilnius"),
        (Some("Kaunas"), None),
    );

    assert_eq!(capitals.get_by_left("Lithuania"), Some(&One::new("Vilnius")));
}
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
