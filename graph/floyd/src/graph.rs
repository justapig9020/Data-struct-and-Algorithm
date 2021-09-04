use config::*;
use std::collections::HashMap;

enum Edge {
    Connect(i32),
    Discon,
}

pub struct Graph {
    edge: Vec<Vec<Edge>>,
}

impl Graph {
    pub from_config(name: &str) -> Graph {
        let mut setting = Config::default();
        setting
            .merge(File::with_name("adjacent.json"))
            .unwrap();
        let mut config = setting
            .try_into::<HashMap<usize, Vec<String>>>()
            .unwrap();
        let mut edges = Vec::with_capacity(config.len());
        for i in 0..config.len() {
            let adj = config.remove(&i).unwrap();
            edges.push(adj);
        }
    }
}
