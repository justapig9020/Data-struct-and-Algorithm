use config::*;
use std::collections::HashMap;

#[derive(Debug, Clone)]
enum Edge {
    Connect(i32),
    Discon,
}

impl Edge {
    fn new(dis: &str) -> Edge {
        match dis.parse::<i32>() {
            Ok(dis) => Edge::Connect(dis),
            Err(_) => Edge::Discon,
        }
    }
    fn to_option(&self) -> Option<i32> {
        match self {
            Edge::Connect(dis) => Some(*dis),
            Edge::Discon => None,
        }
    }
}

#[derive(Debug)]
pub struct Graph {
    edge: Vec<Vec<Edge>>,
}

impl Graph {
    pub fn from_config(name: &str) -> Result<Self, String> {
        let mut setting = Config::default();
        setting
            .merge(File::with_name(name))
            .map_err(|err| err.to_string())?;
        let mut config = setting
            .try_into::<HashMap<usize, Vec<String>>>()
            .map_err(|err| err.to_string())?;
        let mut edges = Vec::with_capacity(config.len());
        for i in 0..config.len() {
            let adj = config
                        .remove(&i)
                        .ok_or(format!("Vertix {} not found", i))?;
            edges.push(adj);
        }
        Ok(Self::new(edges))
    }
    pub fn new(adj: Vec<Vec<String>>) -> Self {
        let vertices = adj.len();
        let mut edge = Vec::with_capacity(vertices);
        for v in adj.iter() {
            let mut adj = Vec::with_capacity(vertices);
            for e in v.iter() {
                adj.push(Edge::new(e));
            }
            edge.push(adj);
        }
        Self {
            edge,
        }
    }
    pub fn distance(&self, a: usize, b: usize) -> Option<i32> {
        let v = self.edge.get(a)?;
        let e = v.get(b)?;
        e.to_option()
    }
    pub fn dump_edges(&self) -> Vec<Vec<Option<i32>>> {
        let edges = self.edge
            .iter()
            .map(|v| v
                .iter()
                .map(|e| e.to_option())
                .collect())
            .collect();
        edges
    }
    pub fn amount(&self) -> usize {
        self.edge.len()
    }
}
