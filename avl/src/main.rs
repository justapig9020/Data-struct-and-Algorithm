mod avl;
mod avl_tree;
use avl_tree::AVLTree;

fn main() -> Result<(), ()> {
    let mut tree = AVLTree::new()?;
    tree.insert_val(10)?;
    tree.insert_val(20)?;
    tree.insert_val(30)?;
    tree.remove_val(20)?;
    tree.inorder();
    Ok(())
}
