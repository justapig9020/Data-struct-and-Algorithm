use graph::Graph;
use std::cmp::min;

pub fn floyd(graph: &Graph) -> Vec<Vec<Option<i32>>> {
    //let mut short = graph.dump_edges();
    let amount = graph.amount();
    let mut short = Vec::with_capacity(amount);
    for i in 0..amount {
        let mut adj = Vec::with_capacity(amount);
        for j in 0..amount {
            adj.push(graph.distance(i, j));
        }
        short.push(adj);
    }

    let len = short.len();
    for i in 0..len {
        for j in 0..len {
            for k in 0..len {
                let ji = short[j][i];
                let ik = short[i][k];
                let relay = ji
                    .and_then(|d1| ik
                        .and_then(|d2| Some(d1 + d2)));
                match (relay, short[j][k]) {
                    (Some(new), Some(old)) => short[j][k] = Some(min(new, old)),
                    (Some(new), None) => short[j][k] = Some(new),
                    (_, _) => {},
                }
            }
        }
    }
    short
}
