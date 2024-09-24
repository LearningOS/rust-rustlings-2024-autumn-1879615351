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
        self.count += 1; //序号从1开始方便处理，因为此时 父节点i的两个子节点为2i 和2i+1，两个子节点都可以通过/2得到父节点的索引
        if self.count < self.items.len() { //当前数组还没有存放满
            self.items[self.count] = value;  //数组默认有空间，没有放满的时候直接放到当前的后一个。
        } else {
            self.items.push(value);//元素加入到数组末尾
        }
        self.heapify_up(self.count); //上浮
    }

    fn heapify_up(&mut self, idx: usize) {
        let mut idx = idx;
        while idx > 1 { //最多找到根节点 ==1
            let parent = self.parent_idx(idx); //当前索引/2找到父节点的索引
            if (self.comparator)(&self.items[idx], &self.items[parent]) { //比较当前节点和父节点
                self.items.swap(idx, parent); //满足条件，交换
                idx = parent; //更新索引，继续上浮
            } else {
                break;
            }
        }
    }

    fn parent_idx(&self, idx: usize) -> usize {
        idx / 2
    }
    //判断是否有孩子，先有左孩子再有右孩子 i,i+1
    fn children_present(&self, idx: usize) -> bool {
        self.left_child_idx(idx) <= self.count
    }

    fn left_child_idx(&self, idx: usize) -> usize {
        idx * 2
    }

    fn right_child_idx(&self, idx: usize) -> usize {
        self.left_child_idx(idx) + 1
    }
    //得到当前节点的满足条件的孩子索引
    fn smallest_child_idx(&self, idx: usize) -> usize {
        let left = self.left_child_idx(idx);
        let right = self.right_child_idx(idx);
        //右孩子的索引较大，判断是否存在右孩子
        if right <= self.count && (self.comparator)(&self.items[right], &self.items[left]) {
            right
        } else {
            left
        }
    }
    //当前最满足条件的元素移除
    fn heapify_down(&mut self, idx: usize) {
        let mut idx = idx;
        while self.children_present(idx) { //当前索引存在孩子
            let child = self.smallest_child_idx(idx); //得到更满足条件的孩子
            if (self.comparator)(&self.items[child], &self.items[idx]) { //判断当前节点和更满足条件的孩子的关系
                self.items.swap(idx, child);
                idx = child;
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
    pub fn new_min() -> Self {
        Self::new(|a, b| a < b)
    }

    pub fn new_max() -> Self {
        Self::new(|a, b| a > b)
    }
}

impl<T> Iterator for Heap<T>
where
    T: Default + Copy,
{
    type Item = T;

    fn next(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }
        let root = self.items[1].clone();
        self.items[1] = self.items[self.count]; //最后一个元素最不满足条件，放在堆顶开始重新调整
        self.count -= 1;  //移除原来最满足条件的，忽略上一步的最不满足条件的元素
        self.heapify_down(1); //重新分配
        Some(root) //返回当前最满足条件的
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