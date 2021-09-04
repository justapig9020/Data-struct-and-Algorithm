mod trans;
use graph::Graph;
use trans::*;

fn main() -> Result<(), String> {
    let g = Graph::from_config("adjacent.json")?;
    let closure = trans_closure(&g);
    print_closure(&closure);
    Ok(())
}
