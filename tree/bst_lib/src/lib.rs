mod bst;
use std::ops::Drop;

pub struct Bst {
    c_bst: *mut bst::BST,
}

#[cfg(test)]
mod bst_test {
    use super::*;

    #[test]
    fn empty() {
        let bst = Bst::new();
        let tree = unsafe { *bst.c_bst };
        assert!(tree.root.is_null());
    }

    #[test]
    fn insert_1() {
        let val = 1;
        let mut bst = Bst::new();
        bst.insert(val);
        let tree = unsafe { *bst.c_bst };
        let root = unsafe { *tree.root };
        assert_eq!(root.val, val);
        assert!(root.left.is_null());
        assert!(root.right.is_null());
    }
}

impl Bst {
    pub fn new() -> Self {
        Self {
            // Safty: The function is implemented by C and return a malloced address. We will later
            // implement Drop for Bst to make sure the allocated memory can be free in the end of
            // Bst's life
            c_bst: unsafe { bst::new_bst() },
        }
    }
    pub fn insert(&mut self, val: i32) {
        unsafe { bst::insert(self.c_bst, val)};
    }
    pub fn remove_val(&mut self, val: i32) -> Option<i32> {
        let rst = unsafe { bst::remove_val(self.c_bst, val) };
        match rst {
            -1 => None,
            val => Some(val),
        }
    }
    pub fn preorder(&self) {
        unsafe { bst::preorder(self.c_bst) };
    }
    pub fn inorder(&self) {
        unsafe { bst::inorder(self.c_bst) };
    }
    pub fn postorder(&self) {
        unsafe { bst::postorder(self.c_bst) };
    }
    pub fn graph(&self) -> Result<String, ()> {
        let tree = self.c_bst;
        if tree.is_null() {
            return Err(());
        }
        let tree = unsafe { *tree };
        if tree.root.is_null() {
            return Err(());
        }
        let root = unsafe { *tree.root };

        let mut graph = String::new();
        let shape =
"node [shape=record];
edge [arrowtail=dot, dir=both, tailclip=false]\n";
        graph.push_str(shape);

        let mut num = 1;
        let body = root.graph(&mut num);
        graph.push_str(body.as_str());

        Ok(graph)
    }
}

impl bst::Node {
    fn graph(&self, num: &mut usize) -> String {
        let mut info = String::new();
        let left = if !self.left.is_null() {
            info.push_str(
                format!("node{:p}:left:c -> node{:p}\n", self, self.left)
                .as_str());
            unsafe { (*self.left).graph(num) }
        } else {
            String::from("")
        };
        info.push_str(
            format!("node{:p} [label=\"{{{{a{} | key: {}}}|{{<left>|<right>}}}}\"]\n", self, num, self.val)
            .as_str());
        *num += 1;
        let right = if !self.right.is_null() {
            info.push_str(
                format!("node{:p}:right:c -> node{:p}\n", self, self.right)
                .as_str());
            unsafe { (*self.right).graph(num) }
        } else {
            String::from("")
        };
        info.push_str(left.as_str());
        info.push_str(right.as_str());
        info
    }
}

impl Drop for Bst {
    fn drop(&mut self) {
        // Safty: Free the c_bst object which we allocated in "new" percedure.
        // The function will free c_bst. It shouldn't access after this operation.
        unsafe { bst::free_bst(self.c_bst) };
    }
}
