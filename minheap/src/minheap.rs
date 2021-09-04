use std::cmp::PartialOrd;

#[derive(Debug)]
struct Minheap<T>
where T: PartialOrd + Copy
{
    heap: Vec<T>,
}

impl<T> Minheap<T>
where T: PartialOrd + Copy
{
    fn new() -> Self {
        Self {
            heap: Vec::new(),
        }
    }
    fn insert(&mut self, val: T) {
        self.heap.push(val);
        let mut child = self.heap.len() - 1;
        while child > 0 {
            let parent = (child - 1) / 2;
            let heap = &mut self.heap;
            if heap[child] < heap[parent] {
                heap.swap(child, parent);
            } else {
                break;
            }
            child = parent;
        }
    }
    fn min(&self) -> Option<T> {
        self.heap.get(0).map(|m| *m)
    }
    fn pop(&mut self) -> Option<T> {
        let heap = &mut self.heap;
        let len = heap.len();
        if len > 1{
            let ret = heap.get(0).map(|m| *m);
            let heir = heap.pop().unwrap();
            heap[0] = heir;
            let mut parent = 0;
            let mut eldest = 1;
            while let Some(old) = heap.get(eldest) {
                let old = *old;
                let (s_posi, small) = if let Some(young) = heap.get(eldest + 1) {
                    let young = *young;
                    if old < young {
                        (eldest, old)
                    } else {
                        (eldest + 1, young)
                    }
                } else {
                    (eldest, old)
                };
                let curr = heap[parent];
                if curr <= small {
                    break;
                } else {
                    heap.swap(parent, s_posi);
                    parent = s_posi;
                    eldest = parent * 2 + 1;
                }
            }
            ret
        } else {
            heap.pop()
        }
    }
}

#[cfg(test)]
mod minheap_test {
    use super::*;
    #[test]
    fn min_test() {
        let arr = [5, 2, 3, 4, 1];
        let mut heap = Minheap::new();
        for i in arr.iter() {
            heap.insert(*i);
        }
        println!("{:?}", heap);
        assert_eq!(1, heap.min().unwrap());
    }

    #[test]
    fn pop_test() {
        let mut arr = [5, 2, 3, 4, 1];
        let mut heap = Minheap::new();
        for i in arr.iter() {
            heap.insert(*i);
        }
        arr.sort();
        for i in arr.iter() {
            assert_eq!(*i, heap.pop().unwrap());
        }
    }
}
