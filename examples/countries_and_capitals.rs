// SPDX-FileCopyrightText: 2025 multi_bimap contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

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

    // Unfortunately, in 1920, Vilnius was annexed by Poland and held until
    // 1939, a regrettable episode in Polish history. During that period
    // Lithuania's capital was Kaunas.
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

    assert_eq!(
        capitals.get_by_left("Lithuania"),
        Some(&One::new("Vilnius"))
    );
}

#[test]
fn test() {
    main();
}
