#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(first_list: &[T], second_list: &[T]) -> Comparison {
    let (left_sublist, superlist, comparison) = order(first_list, second_list);
    if left_sublist.len() == 0 {
        return comparison;
    }
    for right_sublist in sublist_iter(superlist) {
        if matching(left_sublist, right_sublist) {
            return comparison;
        }
    }
    Comparison::Unequal
}

fn order<'a, T>(first_list: &'a [T], second_list: &'a [T]) -> (&'a [T], &'a [T], Comparison) {
    if first_list.len() == second_list.len() {
        (first_list, second_list, Comparison::Equal)
    } else if first_list.len() <= second_list.len() {
        (first_list, second_list, Comparison::Sublist)
    } else {
        (second_list, first_list, Comparison::Superlist)
    }
}

fn matching<T: PartialEq>(sublist: &[T], superlist: &[T]) -> bool {
    if sublist.len() > superlist.len() {
        return false;
    }

    for (i, v) in sublist.iter().enumerate() {
        if v != &superlist[i] {
            return false;
        }
    }

    true
}

struct SubListIter<'a, T> {
    array: &'a [T],
    index: usize,
}

impl<'a, T> Iterator for SubListIter<'a, T> {
    type Item = &'a [T];

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.array.len() {
            self.index = self.index + 1;
            Some(&self.array[(self.index - 1)..])
        } else {
            None
        }
    }
}

fn sublist_iter<T>(array: &[T]) -> SubListIter<T> {
    SubListIter { array, index: 0 }
}
