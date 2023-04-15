use std::collections::BTreeMap;

pub fn transform(h: &BTreeMap<i32, Vec<char>>) -> BTreeMap<char, i32> {
    let mut result = BTreeMap::new();
    for (points, value) in h.iter() {
        for char in value {
            *result.entry(char.to_ascii_lowercase()).or_insert(0) += points;
        }
    }
    result
}
