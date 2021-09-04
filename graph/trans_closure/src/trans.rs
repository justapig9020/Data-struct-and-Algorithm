use graph::Graph;

pub type Closure = Vec<Vec<bool>>;

pub fn trans_closure(graph: &Graph)
    -> Closure {
    //let mut closure = graph.dump_edges();
    let amount = graph.amount();
    let mut closure = Vec::with_capacity(amount);
    for i in 0..amount {
        let mut edge = Vec::with_capacity(amount);
        for j in 0..amount {
            let adj = if i == j {
                false
            } else {
                graph.distance(i, j)
                    .is_some()
            };
            edge.push(adj);
        }
        closure.push(edge);
    }

    let len = closure.len();
    for i in 0..len {
        for j in 0..len {
            for k in 0..len {
                let adj = closure[j][i] && closure[i][k];
                closure[j][k] |= adj;
           }
        }
    }
    closure
}

pub fn print_closure(c: &Closure) {
    for i in 0..c.len() {
        print!("{} -> ", i);
        for j in 0..c.len() {
            if c[i][j] {
                print!("{}, ", j);
            }
        }
        println!("");
    }
}
