use std::{collections::BinaryHeap, iter::FusedIterator};

#[derive(Debug)]
struct Wrapper<T, P> {
    item: T,
    priority: P,
}

impl <T, P> PartialEq for Wrapper<T, P> 
    where P: PartialEq
{
    fn eq(&self, other: &Self) -> bool {
        self.priority.eq(&other.priority)
    }

    fn ne(&self, other: &Self) -> bool {
        self.priority.ne(&other.priority)
    }
}

impl <T, P> Eq for Wrapper<T, P> where P: Eq { }

impl <T, P> PartialOrd for Wrapper<T, P>
    where P: PartialOrd
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        other.priority.partial_cmp(&self.priority)
    }

    fn ge(&self, other: &Self) -> bool {
        other.priority.ge(&self.priority)
    }

    fn gt(&self, other: &Self) -> bool {
        other.priority.gt(&self.priority)
    }

    fn le(&self, other: &Self) -> bool {
        other.priority.le(&self.priority)
    }

    fn lt(&self, other: &Self) -> bool {
        other.priority.lt(&self.priority)
    }
}

impl<T, P> Ord for Wrapper<T, P> 
    where P: Ord
{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.priority.cmp(&self.priority)
    }
}

/// Implements a min PriorityQueue similar to .NET's, where the Item and its Priority are stored separately.
/// Wraps rust's own BinaryHeap, which is stuplidy a max heap.
pub struct PriorityQueue<T, P> {
    heap: BinaryHeap<Wrapper<T, P>>
}

impl<T, P> PriorityQueue<T, P>
    where P: Ord
{
    pub fn new() -> Self {
        Self { 
            heap: BinaryHeap::new()
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            heap: BinaryHeap::with_capacity(capacity)
        }
    }

    pub fn from_vec(v: Vec<(T, P)>) -> Self {
        let mut wrappers = Vec::with_capacity(v.len());
        for (t, p) in v {
            wrappers.push(Wrapper { item: t, priority: p});
        }

        Self {
            heap: BinaryHeap::from(wrappers)
        }
    }

    pub fn enqueue(&mut self, item: T, priority: P) {
        self.heap.push(Wrapper { item: item, priority: priority });
    }

    pub fn dequeue(&mut self) -> Option<T> {
        self.heap.pop().and_then(|w| Some(w.item))
    }

    pub fn dequeue_with_priority(&mut self) -> Option<(T, P)> {
        self.heap.pop().and_then(|w| Some((w.item, w.priority)))
    }

    pub fn peek(&self) -> Option<&T> {
        self.heap.peek().and_then(|w| Some(&w.item))
    }

    pub fn peek_with_priority(&self) -> Option<(&T, &P)> {
        self.heap.peek().and_then(|w| Some((&w.item, &w.priority)))
    }

    pub fn clear(&mut self) {
        self.heap.clear();
    }

    pub fn into_iter_sorted(self) -> IntoIterSorted<T, P> {
        IntoIterSorted { heap: self.heap }
    }
}

impl<T, P> From<Vec<(T, P)>> for PriorityQueue<T, P> 
where
    P: Ord
{
    fn from(value: Vec<(T, P)>) -> Self {
        Self {
            heap: BinaryHeap::from(value.into_iter().map(|(t, p)| { Wrapper { item: t, priority: p}}).collect::<Vec<_>>()),
        }
    }
}

pub struct IntoIterSorted<T, P> {
    heap: BinaryHeap<Wrapper<T, P>>
}

impl<T, P: Ord> Iterator for IntoIterSorted<T, P> {
    type Item = (T, P);

    fn next(&mut self) -> Option<Self::Item> {
        self.heap.pop().and_then(|Wrapper { item: t, priority: p }| Some((t, p)))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let exact = self.heap.len();
        (exact, Some(exact))
    }
}

impl<T, P: Ord> ExactSizeIterator for IntoIterSorted<T, P> { }

impl<T, P: Ord> FusedIterator for IntoIterSorted<T, P> { }