use crate::avl_tree::AVLTree;
use interactive::Management;

pub struct Manager {
    avl: AVLTree,
}

impl Manager {
    pub fn new() -> Result<Self, ()> {
        let avl = AVLTree::new()?;
        Ok(Self {
            avl,
        })
    }
}

fn do_insert_val(avl: &mut AVLTree, args: &[&str]) -> bool {
    if args.len() != 1 {
        return false;
    }
    match args[0].parse::<i32>() {
        Ok(val) => avl.insert_val(val).is_ok(),
        _ => false,
    }
}

fn do_remove_val(avl: &mut AVLTree, args: &[&str]) -> bool {
    if args.len() != 1 {
        return false;
    }
    match args[0].parse::<i32>() {
        Ok(val) => avl.remove_val(val).is_ok(),
        _ => false,
    }
}

impl Management for Manager {
    fn assign_job(&mut self, cmd: &str, args: &[&str]) -> bool {
        let avl = &mut self.avl;
        match cmd {
            "iv" => do_insert_val(avl, args),
            "rv" => do_remove_val(avl, args),
            _ => false,
        }
    }
    fn gen_graph(&self) -> String {
        self.avl.graph().unwrap()
    }
}
