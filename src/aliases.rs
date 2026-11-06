// SPDX-FileCopyrightText: 2026 multi_bimap contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Aliases for multi-bimaps with specific arguments.

use maplike::one::One;

use crate::MultiBimap;

#[cfg(feature = "std")]
/// One-to-one bimap made of two antiparallel hash maps.
pub type HashBimap<L, R> =
    MultiBimap<std::collections::HashMap<L, One<R>>, std::collections::HashMap<R, One<L>>>;
/// One-to-one bimap made of two antiparallel B-tree maps.
pub type BTreeBimap<L, R> =
    MultiBimap<alloc::collections::BTreeMap<L, One<R>>, alloc::collections::BTreeMap<R, One<L>>>;

#[cfg(feature = "std")]
/// Many-to-many bimap made of two antiparallel hash-set-valued hash maps.
pub type HashMultiBimap<L, R> = MultiBimap<
    std::collections::HashMap<L, std::collections::HashSet<R>>,
    std::collections::HashMap<R, std::collections::HashSet<L>>,
>;
#[cfg(feature = "std")]
/// Many-to-many bimap made of two antiparallel hash-set-valued hash maps.
pub type HashHashMultiBimap<L, R> = MultiBimap<
    std::collections::HashMap<L, std::collections::HashSet<R>>,
    std::collections::HashMap<R, std::collections::HashSet<L>>,
>;
#[cfg(feature = "std")]
/// Many-to-many bimap made of two antiparallel B-tree-set-valued hash maps.
pub type HashBTreeMultiBimap<L, R> = MultiBimap<
    std::collections::HashMap<L, alloc::collections::BTreeSet<R>>,
    std::collections::HashMap<R, alloc::collections::BTreeSet<L>>,
>;
#[cfg(feature = "std")]
/// Many-to-many bimap made of two antiparallel vec-valued hash maps.
pub type HashVecMultiBimap<L, R> = MultiBimap<
    std::collections::HashMap<L, alloc::vec::Vec<R>>,
    std::collections::HashMap<R, alloc::vec::Vec<L>>,
>;

/// Many-to-many bimap made of two antiparallel B-tree-set-valued B-tree maps.
pub type BTreeMultiBimap<L, R> = MultiBimap<
    alloc::collections::BTreeMap<L, alloc::collections::BTreeSet<R>>,
    alloc::collections::BTreeMap<R, alloc::collections::BTreeSet<L>>,
>;
/// Many-to-many bimap made of two antiparallel B-tree-set-valued B-tree maps.
pub type BTreeBTreeMultiBimap<L, R> = MultiBimap<
    alloc::collections::BTreeMap<L, alloc::collections::BTreeSet<R>>,
    alloc::collections::BTreeMap<R, alloc::collections::BTreeSet<L>>,
>;
#[cfg(feature = "std")]
/// Many-to-many bimap made of two antiparallel hash-set-valued B-tree maps.
pub type BTreeHashMultiBimap<L, R> = MultiBimap<
    alloc::collections::BTreeMap<L, std::collections::HashSet<R>>,
    alloc::collections::BTreeMap<R, std::collections::HashSet<L>>,
>;
/// Many-to-many bimap made of two antiparallel vec-valued B-tree maps.
pub type BTreeVecMultiBimap<L, R> = MultiBimap<
    alloc::collections::BTreeMap<L, alloc::vec::Vec<R>>,
    alloc::collections::BTreeMap<R, alloc::vec::Vec<L>>,
>;
