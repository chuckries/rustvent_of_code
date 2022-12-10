use std::collections::BinaryHeap;

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
}

impl <T, P> Eq for Wrapper<T, P> where P: Eq { }

impl <T, P> PartialOrd for Wrapper<T, P>
    where P: PartialOrd
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        other.priority.partial_cmp(&self.priority)
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
        PriorityQueue { 
            heap: BinaryHeap::new()
        }
    }

    pub fn enqueue(&mut self, item: T, priority: P) {
        self.heap.push(Wrapper { item: item, priority: priority });
    }

    pub fn dequeue(&mut self) -> Option<T> {
        self.heap.pop().and_then(|w| Some(w.item))
    }

    pub fn try_dequeue(&mut self) -> Option<(T, P)> {
        self.heap.pop().and_then(|w| Some((w.item, w.priority)))
    }

    pub fn peek(&self) -> Option<&T> {
        self.heap.peek().and_then(|w| Some(&w.item))
    }

    pub fn try_peek(&self) -> Option<(&T, &P)> {
        self.heap.peek().and_then(|w| Some((&w.item, &w.priority)))
    }
}