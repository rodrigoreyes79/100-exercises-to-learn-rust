// TODO: Implement a singly linked list, a.k.a. a stack.
//
//  Each node owns the next one through a `Box`, and the list owns the first node (the "head").
//  An empty "next" is represented with `None`:
//
//      head -> [1] -> [2] -> [3] -> None
//
//  Replace every `todo!()` with a working implementation. The tests at the bottom of this file
//  (and in `tests/`) describe the expected behaviour.
//
// Hints:
//  - `Option::take` lets you move a value out of an `Option` you only have a `&mut` to,
//    leaving `None` in its place. You'll use it a lot!
//  - `Option::as_deref` / `Option::as_deref_mut` turn an `&Option<Box<T>>` into an
//    `Option<&T>` (or the `mut` equivalent).
//  - `Option::map` is handy to transform the contents of an `Option` without matching on it.
//  - The compiler-generated `Drop` for this list is *recursive*: dropping the head drops its
//    `Box`, which drops the next node, which drops its `Box`, ...
//    With a long enough list, you'll blow the stack! Write your own `Drop` implementation
//    that frees the nodes iteratively (see `tests/drop.rs`).

pub struct LinkedList<T> {
    head: Link<T>,
    len: usize,
}

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    value: T,
    next: Link<T>,
}

impl<T> LinkedList<T> {
    /// Creates an empty list.
    pub fn new() -> Self {
        LinkedList { head: None, len: 0 }
    }

    /// Returns the number of elements in the list.
    pub fn len(&self) -> usize {
        self.len
    }

    /// Returns `true` if the list contains no elements.
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Adds an element at the front of the list.
    pub fn push_front(&mut self, value: T) {
        let new_node = Node {
            value,
            next: self.head.take(),
        };
        self.head = Some(Box::new(new_node));
        self.len += 1
    }

    /// Removes the first element of the list and returns it, or `None` if the list is empty.
    pub fn pop_front(&mut self) -> Option<T> {
        match self.head.take() {
            None => None,
            Some(n) => {
                self.head = n.next;
                self.len -= 1;
                Some(n.value)
            }
        }
    }

    /// Returns a reference to the first element of the list, or `None` if the list is empty.
    pub fn peek_front(&self) -> Option<&T> {
        self.head.as_deref().map(|node| &node.value)
    }

    /// Returns a mutable reference to the first element of the list,
    /// or `None` if the list is empty.
    pub fn peek_front_mut(&mut self) -> Option<&mut T> {
        self.head.as_deref_mut().map(|node| &mut node.value)
    }

    /// Reverses the order of the elements, in place.
    /// It must not allocate: re-link the existing nodes instead!
    pub fn reverse(&mut self) {
        todo!()
    }

    /// Returns an iterator over references to the elements, front to back.
    pub fn iter(&self) -> Iter<'_, T> {
        todo!()
    }

    /// Returns an iterator over mutable references to the elements, front to back.
    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        todo!()
    }
}

impl<T: PartialEq> LinkedList<T> {
    /// Returns `true` if the list contains an element equal to `value`.
    pub fn contains(&self, value: &T) -> bool {
        todo!()
    }
}

impl<T> Default for LinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}

// TODO: implement `Drop` for `LinkedList<T>`, freeing the nodes one at a time in a loop.

/// Owning iterator, returned by `LinkedList::into_iter`.
pub struct IntoIter<T>(LinkedList<T>);

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

impl<T> IntoIterator for LinkedList<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        todo!()
    }
}

/// Borrowing iterator, returned by `LinkedList::iter`.
pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

impl<'a, T> IntoIterator for &'a LinkedList<T> {
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

/// Mutably borrowing iterator, returned by `LinkedList::iter_mut`.
pub struct IterMut<'a, T> {
    next: Option<&'a mut Node<T>>,
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

impl<'a, T> IntoIterator for &'a mut LinkedList<T> {
    type Item = &'a mut T;
    type IntoIter = IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

/// Builds a list that yields the elements in the same order as the source iterator.
/// E.g. `[1, 2, 3].into_iter().collect::<LinkedList<_>>()` must iterate as `1, 2, 3`.
impl<T> FromIterator<T> for LinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_list_is_empty() {
        let list: LinkedList<i32> = LinkedList::new();
        assert!(list.is_empty());
        assert_eq!(list.len(), 0);
        assert_eq!(list.peek_front(), None);
    }

    #[test]
    fn default_list_is_empty() {
        let list: LinkedList<String> = LinkedList::default();
        assert!(list.is_empty());
    }

    #[test]
    fn push_and_pop() {
        let mut list = LinkedList::new();
        list.push_front(1);
        list.push_front(2);
        list.push_front(3);
        assert_eq!(list.len(), 3);
        assert!(!list.is_empty());

        // Last in, first out
        assert_eq!(list.pop_front(), Some(3));
        assert_eq!(list.pop_front(), Some(2));
        assert_eq!(list.len(), 1);

        list.push_front(4);
        assert_eq!(list.pop_front(), Some(4));
        assert_eq!(list.pop_front(), Some(1));

        assert_eq!(list.pop_front(), None);
        assert_eq!(list.pop_front(), None);
        assert_eq!(list.len(), 0);
        assert!(list.is_empty());
    }

    #[test]
    fn works_with_non_copy_types() {
        let mut list = LinkedList::new();
        list.push_front(String::from("world"));
        list.push_front(String::from("hello"));
        assert_eq!(list.pop_front().as_deref(), Some("hello"));
        assert_eq!(list.pop_front().as_deref(), Some("world"));
    }

    #[test]
    fn peek() {
        let mut list = LinkedList::new();
        assert_eq!(list.peek_front(), None);
        assert_eq!(list.peek_front_mut(), None);

        list.push_front(1);
        list.push_front(2);
        assert_eq!(list.peek_front(), Some(&2));

        if let Some(value) = list.peek_front_mut() {
            *value = 42;
        }
        assert_eq!(list.peek_front(), Some(&42));
        // Peeking must not remove anything
        assert_eq!(list.len(), 2);
        assert_eq!(list.pop_front(), Some(42));
        assert_eq!(list.pop_front(), Some(1));
    }

    #[test]
    fn iter() {
        let mut list = LinkedList::new();
        list.push_front(3);
        list.push_front(2);
        list.push_front(1);

        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), None);

        // `iter` borrows: the list is still usable
        assert_eq!(list.len(), 3);

        // `&LinkedList` can be used in a `for` loop
        let mut sum = 0;
        for value in &list {
            sum += value;
        }
        assert_eq!(sum, 6);
    }

    #[test]
    fn iter_on_empty_list() {
        let list: LinkedList<i32> = LinkedList::new();
        assert_eq!(list.iter().next(), None);
    }

    #[test]
    fn iter_mut() {
        let mut list = LinkedList::new();
        list.push_front(3);
        list.push_front(2);
        list.push_front(1);

        for value in list.iter_mut() {
            *value *= 10;
        }
        for value in &mut list {
            *value += 1;
        }

        let values: Vec<_> = list.iter().copied().collect();
        assert_eq!(values, vec![11, 21, 31]);
    }

    #[test]
    fn into_iter() {
        let mut list = LinkedList::new();
        list.push_front(String::from("c"));
        list.push_front(String::from("b"));
        list.push_front(String::from("a"));

        let values: Vec<String> = list.into_iter().collect();
        assert_eq!(values, vec!["a", "b", "c"]);
    }

    #[test]
    fn from_iterator_preserves_order() {
        let list: LinkedList<i32> = (1..=5).collect();
        assert_eq!(list.len(), 5);
        assert_eq!(list.peek_front(), Some(&1));
        let values: Vec<_> = list.into_iter().collect();
        assert_eq!(values, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn from_empty_iterator() {
        let list: LinkedList<i32> = std::iter::empty().collect();
        assert!(list.is_empty());
    }

    #[test]
    fn contains() {
        let list: LinkedList<&str> = ["a", "b", "c"].into_iter().collect();
        assert!(list.contains(&"a"));
        assert!(list.contains(&"c"));
        assert!(!list.contains(&"z"));

        let empty: LinkedList<&str> = LinkedList::new();
        assert!(!empty.contains(&"a"));
    }

    #[test]
    fn reverse() {
        let mut list: LinkedList<i32> = (1..=4).collect();
        list.reverse();
        assert_eq!(list.len(), 4);
        assert_eq!(list.iter().copied().collect::<Vec<_>>(), vec![4, 3, 2, 1]);

        // Reversing twice gives back the original order
        list.reverse();
        assert_eq!(list.iter().copied().collect::<Vec<_>>(), vec![1, 2, 3, 4]);
    }

    #[test]
    fn reverse_edge_cases() {
        let mut empty: LinkedList<i32> = LinkedList::new();
        empty.reverse();
        assert!(empty.is_empty());

        let mut single: LinkedList<i32> = std::iter::once(7).collect();
        single.reverse();
        assert_eq!(single.pop_front(), Some(7));
        assert_eq!(single.pop_front(), None);
    }
}
