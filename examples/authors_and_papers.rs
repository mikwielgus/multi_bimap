// SPDX-FileCopyrightText: 2025 multi_bimap contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

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

    authorship.insert(
        "Stefan Banach",
        "Sur les opérations dans les ensembles abstraits",
    );
    authorship.insert("Stefan Banach", "Théorie des opérations linéaires");
    authorship.insert(
        "Stefan Banach",
        "Sur le principe de la condensation des singularités",
    );
    authorship.insert(
        "Hugo Steinhaus",
        "Sur le principe de la condensation des singularités",
    );

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

#[test]
fn test() {
    main();
}
