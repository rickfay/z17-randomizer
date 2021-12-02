use std::{cmp::Ordering, collections::BinaryHeap};

type Weight = u32;

#[derive(Clone, Debug)]
pub struct Queue<T>(BinaryHeap<Item<T>>);

impl<T> Queue<T> {
    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn contains(&self, item: &T) -> bool
    where
        T: PartialEq,
    {
        self.0.iter().any(|Item(_, inner)| item == inner)
    }

    pub fn push(&mut self, weight: Weight, item: T) {
        self.0.push(Item(weight, item))
    }

    pub fn merge(&mut self, mut other: Self) {
        while let Some(item) = other.0.pop() {
            self.0.push(item);
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        self.0.pop().map(|Item(_, item)| item)
    }

    pub fn peek(&mut self) -> Option<&T> {
        self.0.peek().map(|Item(_, item)| (item))
    }

    pub fn remove<F>(&mut self, f: F) -> Option<T>
    where
        F: Fn(&T) -> bool,
    {
        self.0.pop().and_then(|item| {
            if f(&item.1) {
                Some(item.1)
            } else {
                let mut retained = vec![item];
                let item = loop {
                    if let Some(Item(weight, item)) = self.0.pop() {
                        if f(&item) {
                            break Some(item);
                        } else {
                            retained.push(Item(weight, item));
                        }
                    } else {
                        break None;
                    }
                };
                self.0.extend(retained);
                item
            }
        })
    }
}

impl<T> Default for Queue<T> {
    fn default() -> Self {
        Self(Default::default())
    }
}

impl<A> Extend<(Weight, A)> for Queue<A> {
    fn extend<T: IntoIterator<Item = (Weight, A)>>(&mut self, iter: T) {
        self.0.extend(iter.into_iter().map(Item::from))
    }
}

#[derive(Clone, Debug)]
pub struct Item<T>(Weight, T);

impl<T> Item<T> {
    fn from(item: (Weight, T)) -> Self {
        Self(item.0, item.1)
    }
}

impl<T> Eq for Item<T> {}

impl<T> PartialEq for Item<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<T> Ord for Item<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.cmp(&other.0)
    }
}

impl<T> PartialOrd for Item<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
