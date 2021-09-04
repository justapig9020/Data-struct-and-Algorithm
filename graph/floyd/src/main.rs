mod floyd;
use graph::Graph;
use floyd::floyd;

fn main() {
    let g = Graph::from_config("adjacent.json")
                .unwrap_or_else(|msg| panic!("{}", msg));
    let adj = floyd(&g);
    for (i, v) in adj.iter().enumerate() {
        for (j, e) in v.iter().enumerate() {
            let dis = e
                    .map(|d| d.to_string())
                    .unwrap_or(String::from("INF"));
            println!("{} -> {}: {}", i, j, dis);
        }
    }
}
