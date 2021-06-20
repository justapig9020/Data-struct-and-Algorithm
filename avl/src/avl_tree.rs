use crate::avl;
use crate::avl::{AVL, Node};
use std::ops::Drop;
use std::ptr;

pub struct AVLTree {
    tree: *mut AVL,
}

impl AVLTree {
    pub fn new() -> Result<Self, ()> {
        let tree = unsafe { avl::new_tree() };
        if tree.is_null() {
            Err(())
        } else {
            Ok(Self {
                tree,
            })
        }
    }
    pub fn insert_val(&mut self, val: i32) -> Result<i32, ()> {
        let result =
            unsafe { avl::insert_val(self.tree, val) };
        if result {
            Ok(val)
        } else {
            Err(())
        }
    }
    pub fn remove_val(&mut self, val: i32) -> Result<i32, ()> {
        let result =
            unsafe { avl::remove_val(self.tree, val) };
        if result {
            Ok(val)
        } else {
            Err(())
        }
    }
    pub fn inorder(&self) {
        unsafe { avl::inorder(self.tree) }
    }
    pub fn graph(&self) {
        println!(
"digraph LinkedList {{
node [shape=record];
edge [arrowtail=dot, dir=both, tailclip=false]");
        unsafe {
            let tree = (*self).tree;
            let root = (*tree).root;
            (*root).graph();
        }
        println!("}}");
    }
}

impl Node {
    fn graph(&self) {
        println!("node{:p} [label=\"{{<val>{}|{{<left>|<right>}}}}\"]", self, self.val);
        if !self.left.is_null() {
            println!("node{:p}:left:c -> node{:p}", self, self.left);
            unsafe { (*self.left).graph() };
        }
        if !self.right.is_null() {
            println!("node{:p}:right:c -> node{:p}", self, self.right);
            unsafe { (*self.right).graph() };
        }
    }
}

impl Drop for AVLTree {
    fn drop(&mut self) {
        unsafe { avl::del_tree(self.tree) };
        self.tree = ptr::null_mut();
    }
}

