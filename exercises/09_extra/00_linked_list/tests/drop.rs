// These tests live in their own file (and therefore their own test binary) because,
// without a custom `Drop` implementation, `dropping_a_long_list_does_not_overflow_the_stack`
// crashes the whole process with a stack overflow, taking every other test down with it.
use linked_list::LinkedList;
use std::cell::Cell;
use std::rc::Rc;

#[test]
fn dropping_a_long_list_does_not_overflow_the_stack() {
    let mut list = LinkedList::new();
    for i in 0..1_000_000 {
        list.push_front(i);
    }
    assert_eq!(list.len(), 1_000_000);
    drop(list);
}

/// Increments a shared counter when dropped.
struct DropCounter(Rc<Cell<usize>>);

impl Drop for DropCounter {
    fn drop(&mut self) {
        self.0.set(self.0.get() + 1);
    }
}

#[test]
fn every_element_is_dropped_exactly_once() {
    let drops = Rc::new(Cell::new(0));
    let mut list = LinkedList::new();
    for _ in 0..10 {
        list.push_front(DropCounter(drops.clone()));
    }

    // Popped elements are owned by the caller now
    drop(list.pop_front());
    assert_eq!(drops.get(), 1);

    drop(list);
    assert_eq!(drops.get(), 10);
}

#[test]
fn partially_consumed_into_iter_drops_the_rest() {
    let drops = Rc::new(Cell::new(0));
    let list: LinkedList<_> = (0..5).map(|_| DropCounter(drops.clone())).collect();

    let mut iter = list.into_iter();
    drop(iter.next());
    assert_eq!(drops.get(), 1);

    drop(iter);
    assert_eq!(drops.get(), 5);
}
