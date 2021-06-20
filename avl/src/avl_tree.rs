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
    pub fn graph(&self) -> Result<String, ()> {
        let tree = self.tree;
        if tree.is_null() {
            return Err(());
        }
        let tree = unsafe { *tree };
        if tree.root.is_null() {
            return Err(());
        }
        let root = unsafe { *tree.root };

        let mut graph = String::new();
        let header =
"digraph LinkedList {
node [shape=record];
edge [arrowtail=dot, dir=both, tailclip=false]\n";
        graph.push_str(header);

        let body = root.graph();
        graph.push_str(body.as_str());

        let footer = "}\n";
        graph.push_str(footer);
        Ok(graph)
    }
}

impl Node {
    fn graph(&self) -> String {
        let mut info =
            format!("node{:p} [label=\"{{<val>val: {}|height: {}|{{<left>|<right>}}}}\"]\n", self, self.val, self.height);
        let left = if !self.left.is_null() {
            info.push_str(
                format!("node{:p}:left:c -> node{:p}\n", self, self.left)
                .as_str());
            unsafe { (*self.left).graph() }
        } else {
            String::from("")
        };
        let right = if !self.right.is_null() {
            info.push_str(
                format!("node{:p}:right:c -> node{:p}\n", self, self.right)
                .as_str());
            unsafe { (*self.right).graph() }
        } else {
            String::from("")
        };
        info.push_str(left.as_str());
        info.push_str(right.as_str());
        info
    }
}

impl Drop for AVLTree {
    fn drop(&mut self) {
        unsafe { avl::del_tree(self.tree) };
        self.tree = ptr::null_mut();
    }
}

