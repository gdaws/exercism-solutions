use core::cmp::Ord;
use std::{cmp::Ordering, convert::AsRef};

pub fn find<T: Ord, C: AsRef<[T]>>(array: C, key: T) -> Option<usize> {
    binary_search(0, array.as_ref(), key)
}

fn binary_search<T: Ord>(begin: usize, array: &[T], key: T) -> Option<usize> {
    if array.len() == 0 {
        return None;
    }
    let middle = array.len() / 2;

    match key.cmp(&array[middle]) {
        Ordering::Equal => Some(begin + middle),
        Ordering::Less => binary_search(begin, &array[..middle], key),
        Ordering::Greater => binary_search(begin + middle + 1, &array[middle + 1..], key),
    }
}
