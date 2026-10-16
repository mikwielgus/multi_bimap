// SPDX-FileCopyrightText: 2026 multi_bimap contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use multi_bimap::HashMultiBimap;
use std::collections::HashSet;
use std::rc::Rc;

fn main() {
    // Polish medieval and Renaissance scientists and the fields they worked in.
    // This is a many-to-many relation: some scientists worked in multiple
    // fields, and some fields were worked on by multiple scientists.
    let mut fields: HashMultiBimap<Rc<str>, Rc<str>> = HashMultiBimap::new();

    let vitello = Rc::from("Vitello");
    let krol = Rc::from("Marcin Król of Żurawica");
    let brudzewski = Rc::from("Albert Brudzewski");
    let copernicus = Rc::from("Nicolaus Copernicus");
    let sedziwoj = Rc::from("Michał Sędziwój");

    let optics = Rc::from("optics");
    let astronomy = Rc::from("astronomy");
    let mathematics = Rc::from("mathematics");
    let alchemy = Rc::from("alchemy");

    // Vitello was both an optician and mathematician.
    fields.insert(Rc::clone(&vitello), Rc::clone(&optics));
    fields.insert(Rc::clone(&vitello), Rc::clone(&mathematics));

    // Król, Brudzewski, and Copernicus were both astronomers and
    // mathematicians.
    fields.insert(Rc::clone(&krol), Rc::clone(&astronomy));
    fields.insert(Rc::clone(&krol), Rc::clone(&mathematics));
    fields.insert(Rc::clone(&brudzewski), Rc::clone(&astronomy));
    fields.insert(Rc::clone(&brudzewski), Rc::clone(&mathematics));
    fields.insert(Rc::clone(&copernicus), Rc::clone(&astronomy));
    fields.insert(Rc::clone(&copernicus), Rc::clone(&mathematics));

    // Sędziwój was an alchemist.
    fields.insert(Rc::clone(&sedziwoj), Rc::clone(&alchemy));

    assert_eq!(
        fields.get_by_left("Albert Brudzewski"),
        Some(&HashSet::from([
            Rc::clone(&astronomy),
            Rc::clone(&mathematics),
        ])),
    );
    assert_eq!(
        fields.get_by_right("astronomy"),
        Some(&HashSet::from([
            Rc::clone(&krol),
            Rc::clone(&brudzewski),
            Rc::clone(&copernicus),
        ])),
    );
}

#[test]
fn test() {
    main();
}
