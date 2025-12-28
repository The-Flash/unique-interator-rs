use std::{collections::HashSet, hash::Hash};

pub trait IteratorExt: Iterator {
    fn unique(self) -> UniqueIterator<Self> where 
        Self: Sized,
        Self::Item: Eq + Hash + Clone
    {
        UniqueIterator { iter: self, seen: HashSet::new(), }
    }
}

pub struct UniqueIterator<I> where I: Iterator {
    iter: I,
    seen: HashSet<I::Item>
}

impl<I> Iterator for UniqueIterator<I> where 
    I:Iterator,
    I::Item: Eq + Hash + Clone
{
    type Item = I::Item;
    fn next(&mut self) -> Option<Self::Item> {
       self.iter.find(|item| self.seen.insert(item.clone()))
    }
}

impl<I: Iterator> IteratorExt for I {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
         let numbers = vec![1, 2, 3, 4, 4, 6, 2];
         let unique_numbers: Vec<_> = numbers.into_iter().unique().collect();
         println!("{:?}", unique_numbers);
         assert_eq!(unique_numbers, vec![1, 2, 3, 4, 6]);
    }
}
