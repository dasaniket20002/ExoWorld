use std::{cmp::Ordering, slice};

#[derive(Debug)]
pub struct SortedVec<T>(pub Vec<T>);

impl<T> SortedVec<T> {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    // #[inline]
    // pub fn capacity(&self) -> usize {
    //     self.0.capacity()
    // }

    // #[inline]
    // pub fn get(&self, i: usize) -> &T {
    //     unsafe { self.0.get_unchecked(i) }
    // }

    // pub fn get_vec(&self) -> &Vec<T> {
    //     &self.0
    // }

    // #[inline]
    // pub fn get_mut(&mut self, i: usize) -> &mut T {
    //     unsafe { self.0.get_unchecked_mut(i) }
    // }

    #[inline]
    pub fn push(&mut self, item: T) -> usize {
        self.0.push(item);
        self.len() - 1
    }

    pub fn insert_sorted_by<F>(&mut self, item: T, mut cmp: F) -> usize
    where
        F: FnMut(&T, &T) -> Ordering,
    {
        unsafe {
            let idx = self.0[..self.len()]
                .binary_search_by(|x| cmp(x, &item))
                .unwrap_or_else(|i| i);

            if self.0.len() == self.0.capacity() {
                self.0.reserve(1);
            }

            let ptr = self.0.as_mut_ptr();

            // Shift initialized elements [idx..len) one slot to the right.
            //
            // ptr::copy is equivalent to memmove and correctly handles overlapping
            // regions.
            std::ptr::copy(ptr.add(idx), ptr.add(idx + 1), self.len() - idx);

            // Construct the new element into the hole.
            (*ptr.add(idx)) = item;

            self.0.set_len(self.0.len() + 1);
            idx
        }
    }

    // #[inline]
    // pub fn swap_remove(&mut self, idx: usize) {
    //     self.0.swap_remove(idx);
    // }

    // pub fn remove_sorted(&mut self, idx: usize) {
    //     unsafe {
    //         let ptr = self.0.as_mut_ptr();

    //         // Shift [idx + 1 .. len) left by one.
    //         std::ptr::copy(ptr.add(idx + 1), ptr.add(idx), self.len() - idx - 1);
    //     }
    //     self.0.pop();
    // }

    #[inline]
    pub fn clear(&mut self) {
        self.0.clear();
    }

    #[inline]
    pub fn iter(&self) -> slice::Iter<'_, T> {
        self.0.iter()
    }

    // #[inline]
    // pub fn iter_mut(&mut self) -> slice::IterMut<'_, T> {
    //     self.0.iter_mut()
    // }
}
