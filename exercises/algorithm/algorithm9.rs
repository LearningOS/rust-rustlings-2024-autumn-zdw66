/*
	heap
	This question requires you to implement a binary heap function
*/


use std::cmp::Ord;
use std::default::Default;

pub struct Heap<T>
where
    T: Default,
{
    count: usize,
    items: Vec<T>,
    comparator: fn(&T, &T) -> bool,
}

impl<T> Heap<T>
where
    T: Default,
{
    pub fn new(comparator: fn(&T, &T) -> bool) -> Self {
        Self {
            count: 0,
            items: vec![T::default()],
            comparator,
        }
    }

    pub fn len(&self) -> usize {
        self.count
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn add(&mut self, value: T) {
        //TODO
        self.items.push(T::default());
        let idx = self.count;
        self.count += 1;
        self.items[idx] = value;
        self.sift_up(idx);
    }

    fn parent_idx(&self, idx: usize) -> usize {
        idx / 2
    }

    fn children_present(&self, idx: usize) -> bool {
        self.left_child_idx(idx) <= self.count
    }

    fn left_child_idx(&self, idx: usize) -> usize {
        idx * 2
    }

    fn right_child_idx(&self, idx: usize) -> usize {
        self.left_child_idx(idx) + 1
    }

    fn smallest_child_idx(&self, idx: usize) -> usize {
        //TODO
		let left = self.left_child_idx(idx);
        let right = self.right_child_idx(idx);

        if left < self.count && right < self.count {
            if (self.comparator)(&self.items[right], &self.items[left]) {
                right
            } else {
                left
            }
        } else if left < self.count {
            left
        } else if right < self.count {
            right
        } else {
            0  // No children
        }
    }

    fn sift_up(&mut self, mut idx: usize) {
        while idx > 0 {
            let parent = self.parent_idx(idx);
            if !(self.comparator)(&self.items[parent], &self.items[idx]) {
                self.items.swap(parent, idx);
                idx = parent;
            } else {
                break;
            }
        }
    }
    //子节点是否小于父节点,如果 self.comparator(&self.items[idx], &self.items[parent]) 返回 true，这意味着子节点是较小值
    // fn sift_up(&mut self, mut idx: usize) {
    //     while idx > 0 {
    //         let parent = self.parent_idx(idx);
    //         if (self.comparator)(&self.items[idx], &self.items[parent]) {
    //             self.items.swap(parent, idx);
    //             idx = parent;
    //         } else {
    //             break;
    //         }
    //     }
    // }

    fn sift_down(&mut self, mut idx: usize) {
        loop {  
            let left = self.left_child_idx(idx);  
            let right = self.right_child_idx(idx);  
            let mut smallest_or_largest = idx;  
    
            if left < self.count && (self.comparator)(&self.items[left], &self.items[smallest_or_largest]) {  
                smallest_or_largest = left;  
            }  
            if right < self.count && (self.comparator)(&self.items[right], &self.items[smallest_or_largest]) {  
                smallest_or_largest = right;  
            }  
            if smallest_or_largest != idx {  
                self.items.swap(smallest_or_largest, idx);  
                idx = smallest_or_largest;  
            } else {  
                break;  
            }  
        }
    }
    
}

impl<T> Heap<T>
where
    T: Default + Ord,
{
    /// Create a new MinHeap
    pub fn new_min() -> Heap<T> {
        Self::new(|a, b| a < b)
    }

    /// Create a new MaxHeap
    pub fn new_max() -> Heap<T> {
        Self::new(|a, b| a > b)
    }
}

impl<T> Iterator for Heap<T>
where
    T: Default+ Clone,
{
    type Item = T;

    fn next(&mut self) -> Option<T> {
        //TODO
		if self.count == 0 {
            None
        } else {   
            let result = self.items[0].clone();
            self.items[0] = self.items[self.count - 1].clone();
            self.count -= 1;
            self.sift_down(0);
            Some(result)
        }
    }
}

pub struct MinHeap;

impl MinHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord,
    {
        Heap::new(|a, b| a < b)
    }
}

pub struct MaxHeap;

impl MaxHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord,
    {
        Heap::new(|a, b| a > b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_empty_heap() {
        let mut heap = MaxHeap::new::<i32>();
        assert_eq!(heap.next(), None);
    }

    #[test]
    fn test_min_heap() {
        let mut heap = MinHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(2));
        assert_eq!(heap.next(), Some(4));
        assert_eq!(heap.next(), Some(9));
        heap.add(1);
        assert_eq!(heap.next(), Some(1));
    }

    #[test]
    fn test_max_heap() {
        let mut heap = MaxHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(11));
        assert_eq!(heap.next(), Some(9));
        assert_eq!(heap.next(), Some(4));
        heap.add(1);
        assert_eq!(heap.next(), Some(2));
    }
}