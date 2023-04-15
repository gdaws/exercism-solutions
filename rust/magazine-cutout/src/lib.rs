// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

use std::collections::HashMap;

pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {
    let mut available = HashMap::new();

    for word in magazine.iter() {
        *available.entry(*word).or_insert(0) += 1;
    }

    for word in note.iter() {
        match available.get_mut(*word) {
            None | Some(0) => return false,
            Some(counter) => {
                *counter -= 1;
            }
        };
    }

    true
}
