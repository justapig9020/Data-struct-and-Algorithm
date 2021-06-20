mod avl;
mod avl_tree;
use avl_tree::AVLTree;

fn main() -> Result<(), ()> {
    let mut tree = AVLTree::new()?;
    let insert = [100, 30, 50, 105, 110, 20, 25, 15, 23, 16, 14, 13];
    for val in insert.iter() {
        tree.insert_val(*val)?;
    }
    let rm = [105, 25, 23];
    for val in rm.iter() {
        tree.remove_val(*val)?;
    }
    println!("{}", tree.graph()?);
    Ok(())
}
