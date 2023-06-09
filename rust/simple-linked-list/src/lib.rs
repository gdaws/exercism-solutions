use std::iter::FromIterator;

pub struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    fn new(data: T) -> Box<Node<T>> {
        Box::new(Node { data, next: None })
    }
}

pub struct NodeIterator<'a, T> {
    head: &'a Option<Box<Node<T>>>,
}

impl<'a, T> Iterator for NodeIterator<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        match self.head {
            Some(node) => {
                self.head = &node.next;
                Some(&node.data)
            }
            None => None,
        }
    }
}

pub struct SimpleLinkedList<T> {
    head: Option<Box<Node<T>>>,
}

impl<T> SimpleLinkedList<T> {
    pub fn new() -> Self {
        SimpleLinkedList { head: None }
    }

    // You may be wondering why it's necessary to have is_empty()
    // when it can easily be determined from len().
    // It's good custom to have both because len() can be expensive for some types,
    // whereas is_empty() is almost always cheap.
    // (Also ask yourself whether len() is expensive for SimpleLinkedList)
    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    pub fn len(&self) -> usize {
        self.iter().count()
    }

    pub fn push(&mut self, data: T) {
        let old_head = self.head.replace(Node::new(data));
        if let Some(node) = &mut self.head {
            node.next = old_head;
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        let detached_head = self.head.take();
        match detached_head {
            Some(node) => {
                self.head = node.next;
                Some(node.data)
            }
            None => None,
        }
    }

    pub fn peek(&self) -> Option<&T> {
        match &self.head {
            Some(node) => Some(&node.data),
            None => None,
        }
    }

    #[must_use]
    pub fn rev(mut self) -> SimpleLinkedList<T> {
        let mut rev_list = SimpleLinkedList::new();
        while !self.is_empty() {
            if let Some(value) = self.pop() {
                rev_list.push(value);
            }
        }
        rev_list
    }

    pub fn iter(&self) -> NodeIterator<T> {
        NodeIterator { head: &self.head }
    }
}

impl<T> FromIterator<T> for SimpleLinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut list = SimpleLinkedList::new();
        for value in iter {
            list.push(value);
        }
        list
    }
}

// In general, it would be preferable to implement IntoIterator for SimpleLinkedList<T>
// instead of implementing an explicit conversion to a vector. This is because, together,
// FromIterator and IntoIterator enable conversion between arbitrary collections.
// Given that implementation, converting to a vector is trivial:
//
// let vec: Vec<_> = simple_linked_list.into_iter().collect();
//
// The reason this exercise's API includes an explicit conversion to Vec<T> instead
// of IntoIterator is that implementing that interface is fairly complicated, and
// demands more of the student than we expect at this point in the track.

impl<T> From<SimpleLinkedList<T>> for Vec<T> {
    fn from(mut linked_list: SimpleLinkedList<T>) -> Vec<T> {
        let mut v = Vec::new();
        while !linked_list.is_empty() {
            if let Some(value) = linked_list.pop() {
                v.push(value);
            }
        }
        v.reverse();
        v
    }
}
