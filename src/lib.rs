// SPDX-FileCopyrightText: 2025 multi_bimap contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#![doc(html_root_url = "https://docs.rs/multi_bimap")]
#![doc = include_str!("../README.md")]
#![deny(missing_docs)]
#![forbid(unsafe_code)]
//#![no_std]

use maplike::containers::Container;
use maplike::ops::{Clear, Get, Insert, Modify, Put, Remove, WithOne};

/// Many-to-many bidirectional map made of two antiparallel maps.
pub struct Bimap<L2R, R2L> {
    left_to_right: L2R,
    right_to_left: R2L,
}

impl<L2R, R2L> Bimap<L2R, R2L> {
    /// Returns a reference to the left-to-right map.
    pub fn left_to_right(&self) -> &L2R {
        &self.left_to_right
    }

    /// Returns a reference to the right-to-left map.
    pub fn right_to_left(&self) -> &R2L {
        &self.right_to_left
    }
}

impl<L2R: Default, R2L: Default> Bimap<L2R, R2L> {
    /// Creates a new, empty `Bimap`.
    pub fn new() -> Self {
        Bimap {
            left_to_right: Default::default(),
            right_to_left: Default::default(),
        }
    }
}

impl<L2R, R2L> Container for Bimap<L2R, R2L>
where
    L2R: Container,
    R2L: Container,
{
    type Key = <L2R as Container>::Key;
    type Value = <R2L as Container>::Key;
}

impl<L2R, R2L> Insert<<L2R as Container>::Key> for Bimap<L2R, R2L>
where
    L2R: Container,
    R2L: Container,
    L2R: Get<<L2R as Container>::Key>
        + Insert<<L2R as Container>::Key>
        + Modify<<L2R as Container>::Key>,
    R2L: Get<<R2L as Container>::Key>
        + Insert<<R2L as Container>::Key>
        + Modify<<R2L as Container>::Key>,
    <L2R as Container>::Value: WithOne<<R2L as Container>::Key> + Put<<R2L as Container>::Key>,
    <R2L as Container>::Value: WithOne<<L2R as Container>::Key> + Put<<L2R as Container>::Key>,
    <L2R as Container>::Key: Clone,
    <R2L as Container>::Key: Clone,
{
    type Output = (
        Option<<R2L as Container>::Key>,
        Option<<L2R as Container>::Key>,
    );

    fn insert(
        &mut self,
        key: <L2R as Container>::Key,
        value: <R2L as Container>::Key,
    ) -> Self::Output {
        Bimap::insert(self, key, value)
    }
}

impl<L2R, R2L> Bimap<L2R, R2L>
where
    L2R: Container,
    R2L: Container,
    L2R: Get<<L2R as Container>::Key>
        + Insert<<L2R as Container>::Key>
        + Modify<<L2R as Container>::Key>,
    R2L: Get<<R2L as Container>::Key>
        + Insert<<R2L as Container>::Key>
        + Modify<<R2L as Container>::Key>,
    <L2R as Container>::Value: WithOne<<R2L as Container>::Key> + Put<<R2L as Container>::Key>,
    <R2L as Container>::Value: WithOne<<L2R as Container>::Key> + Put<<L2R as Container>::Key>,
{
    /// Insert a left-right association into the bimap.
    ///
    /// Both sides may map to multiple values.
    ///
    /// Returns any values that have been displaced on each side by this
    /// insertion.
    ///
    /// # Examples
    ///
    /// ```
    /// use multi_bimap::Bimap;
    /// use std::collections::{HashMap, HashSet};
    ///
    /// let mut m: Bimap<HashMap<&str, HashSet<i32>>, HashMap<i32, HashSet<&str>>> = Bimap::new();
    ///
    /// assert_eq!(m.insert("a", 1), (None, None));
    /// m.insert("a", 2);
    /// m.insert("b", 1);
    ///
    /// let mut a_rights: Vec<_> = m.left_to_right().get(&"a").unwrap().iter().copied().collect();
    /// a_rights.sort();
    /// assert_eq!(a_rights, [1, 2]);
    /// ```
    pub fn insert(
        &mut self,
        left: <L2R as Container>::Key,
        right: <R2L as Container>::Key,
    ) -> (
        Option<<R2L as Container>::Key>,
        Option<<L2R as Container>::Key>,
    )
    where
        <L2R as Container>::Key: Clone,
        <R2L as Container>::Key: Clone,
    {
        // PERF: Using Entry API may be faster here, but not all collections
        // support it.

        let left_out = if self.left_to_right.get(&left).is_some() {
            let mut out = None;

            self.left_to_right.modify(&left, |rights| {
                out = rights.put(right.clone());
            });

            out
        } else {
            self.left_to_right
                .insert(left.clone(), WithOne::with_one(right.clone()));
            None
        };

        let right_out = if self.right_to_left.get(&right).is_some() {
            let mut out = None;
            self.right_to_left.modify(&right, |lefts| {
                out = lefts.put(left.clone());
            });
            out
        } else {
            self.right_to_left.insert(right, WithOne::with_one(left));
            None
        };

        (left_out, right_out)
    }
}

impl<L2R, R2L> Remove<(<L2R as Container>::Key, <R2L as Container>::Key)> for Bimap<L2R, R2L>
where
    L2R: Container,
    R2L: Container,
    L2R: Get<<L2R as Container>::Key>
        + Modify<<L2R as Container>::Key>
        + Remove<<L2R as Container>::Key>,
    R2L: Get<<R2L as Container>::Key>
        + Modify<<R2L as Container>::Key>
        + Remove<<R2L as Container>::Key>,
    <L2R as Container>::Value: Remove<<R2L as Container>::Key, Output = bool> + Default + PartialEq,
    <R2L as Container>::Value: Remove<<L2R as Container>::Key, Output = bool> + Default + PartialEq,
    <L2R as Container>::Key: Clone,
    <R2L as Container>::Key: Clone,
{
    type Output = Option<(<L2R as Container>::Key, <R2L as Container>::Key)>;

    fn remove(
        &mut self,
        key: &(<L2R as Container>::Key, <R2L as Container>::Key),
    ) -> Option<(<L2R as Container>::Key, <R2L as Container>::Key)> {
        Bimap::remove(self, &key.0, &key.1)
    }
}

impl<L2R, R2L> Bimap<L2R, R2L>
where
    L2R: Container,
    R2L: Container,
    L2R: Get<<L2R as Container>::Key>
        + Modify<<L2R as Container>::Key>
        + Remove<<L2R as Container>::Key>,
    R2L: Get<<R2L as Container>::Key>
        + Modify<<R2L as Container>::Key>
        + Remove<<R2L as Container>::Key>,
    <L2R as Container>::Value: Remove<<R2L as Container>::Key, Output = bool> + Default + PartialEq,
    <R2L as Container>::Value: Remove<<L2R as Container>::Key, Output = bool> + Default + PartialEq,
    <L2R as Container>::Key: Clone,
    <R2L as Container>::Key: Clone,
{
    /// Remove a left-right association from the bimap.
    ///
    /// Empty keys are dropped from both sides.
    ///
    /// Returns the removed pair if it was present.
    ///
    /// # Examples
    ///
    /// ```
    /// use multi_bimap::Bimap;
    /// use std::collections::{HashMap, HashSet};
    ///
    /// let mut m: Bimap<HashMap<&str, HashSet<i32>>, HashMap<i32, HashSet<&str>>> = Bimap::new();
    ///
    /// m.insert("a", 1);
    /// m.insert("a", 2);
    /// m.insert("b", 1);
    ///
    /// assert_eq!(m.remove(&"a", &1), Some(("a", 1)));
    /// assert_eq!(m.remove(&"a", &1), None);
    ///
    /// let mut a_rights: Vec<_> = m.left_to_right().get(&"a").unwrap().iter().copied().collect();
    /// a_rights.sort();
    /// assert_eq!(a_rights, [2]);
    /// ```
    pub fn remove(
        &mut self,
        left: &<L2R as Container>::Key,
        right: &<R2L as Container>::Key,
    ) -> Option<(<L2R as Container>::Key, <R2L as Container>::Key)> {
        let mut present = false;

        if self.left_to_right.get(left).is_some() {
            self.left_to_right.modify(left, |rights| {
                present = rights.remove(right);
            });

            if present && self.left_to_right.get(left) == Some(&Default::default()) {
                self.left_to_right.remove(left);
            }
        }

        if self.right_to_left.get(right).is_some() {
            let mut present_right = false;

            self.right_to_left.modify(right, |lefts| {
                present_right = lefts.remove(left);
            });

            if present_right && self.right_to_left.get(right) == Some(&Default::default()) {
                self.right_to_left.remove(right);
            }
        }

        present.then(|| (left.clone(), right.clone()))
    }
}

impl<L2R, R2L> Clear for Bimap<L2R, R2L>
where
    L2R: Clear,
    R2L: Clear,
{
    fn clear(&mut self) {
        Bimap::clear(self)
    }
}

impl<L2R, R2L> Bimap<L2R, R2L>
where
    L2R: Clear,
    R2L: Clear,
{
    /// Remove all associations from the bimap.
    ///
    /// Clears both the left-to-right and right-to-left sides.
    ///
    /// # Examples
    ///
    /// ```
    /// use multi_bimap::Bimap;
    /// use std::collections::{HashMap, HashSet};
    ///
    /// let mut m: Bimap<HashMap<&str, HashSet<i32>>, HashMap<i32, HashSet<&str>>> = Bimap::new();
    ///
    /// m.insert("a", 1);
    /// m.insert("b", 2);
    /// m.clear();
    ///
    /// assert!(m.left_to_right().is_empty());
    /// assert!(m.right_to_left().is_empty());
    /// ```
    pub fn clear(&mut self) {
        self.left_to_right.clear();
        self.right_to_left.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::{HashMap, HashSet};

    fn sorted_values<T: Copy + Ord>(set: &HashSet<T>) -> Vec<T> {
        let mut v: Vec<_> = set.iter().copied().collect();
        v.sort();
        v
    }

    #[test]
    fn insert_allows_many_on_both_sides() {
        let mut m: Bimap<HashMap<&str, HashSet<i32>>, HashMap<i32, HashSet<&str>>> = Bimap::new();

        m.insert("a", 1);
        m.insert("a", 2);
        m.insert("b", 1);

        assert_eq!(
            m.left_to_right().get(&"a").map(sorted_values),
            Some(vec![1, 2])
        );
        assert_eq!(
            m.left_to_right().get(&"b").map(sorted_values),
            Some(vec![1])
        );
        assert_eq!(
            m.right_to_left().get(&1).map(sorted_values),
            Some(vec!["a", "b"]),
        );
        assert_eq!(
            m.right_to_left().get(&2).map(sorted_values),
            Some(vec!["a"])
        );
    }

    #[test]
    fn remove_pair_and_drop_empty_keys() {
        let mut m: Bimap<HashMap<&str, HashSet<i32>>, HashMap<i32, HashSet<&str>>> = Bimap::new();

        m.insert("a", 1);
        m.insert("a", 2);
        m.insert("b", 1);

        assert_eq!(m.remove(&"a", &1), Some(("a", 1)));
        assert_eq!(m.remove(&"a", &1), None);

        assert_eq!(
            m.left_to_right().get(&"a").map(sorted_values),
            Some(vec![2])
        );
        assert_eq!(
            m.right_to_left().get(&1).map(sorted_values),
            Some(vec!["b"])
        );

        assert_eq!(m.remove(&"a", &2), Some(("a", 2)));
        assert!(m.left_to_right().get(&"a").is_none());
        assert!(m.right_to_left().get(&2).is_none());
    }

    #[test]
    fn clear_empties_both_sides() {
        let mut m: Bimap<HashMap<&str, HashSet<i32>>, HashMap<i32, HashSet<&str>>> = Bimap::new();

        m.insert("a", 1);
        m.insert("b", 2);
        m.clear();

        assert!(m.left_to_right().is_empty());
        assert!(m.right_to_left().is_empty());
    }
}
