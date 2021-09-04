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
struct Vertex {
    edge: Vec<Edge>,
    indegree: usize,
    outdegree: usize,
}

impl Vertex {
    fn new(v: usize, t: usize) -> Self {
        let mut edge = vec![Edge::Discon; t];
        edge[v] = Edge::Connect(0);
        Self {
            edge,
            indegree: 0,
            outdegree: 0,
        }
    }
    fn new_edge(&mut self, adj: usize, dis: i32) {
        self.edge[adj] = Edge::Connect(dis);
        self.outdegree += 1;
    }
    fn to(&self, distination: usize) -> Option<&Edge> {
        self.edge.get(distination)
    }
}

#[derive(Debug)]
pub struct Graph {
    vertices: Vec<Vertex>,
}

impl Graph {
    /// Generate a graph by given a name of json file
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
        let amount = adj.len();
        let mut vertices = Vec::with_capacity(amount);
        for i in 0..amount {
            vertices.push(Vertex::new(i, amount));
        }
        for (i, v) in adj.iter().enumerate() {
            for (j, e) in v.iter().enumerate() {
                if let Ok(dis) = e.parse::<i32>() {
                    vertices[i].new_edge(j, dis);
                }
            }
        }
        Self {
            vertices,
        }
    }

    /// Get distance of the edge from "a" to "b"
    pub fn distance(&self, a: usize, b: usize) -> Option<i32> {
        let v = self.vertices.get(a)?;
        let e = v.to(b)?;
        e.to_option()
    }

    /// Return edges in adjacent matrix form.
    pub fn dump_edges(&self) -> Vec<Vec<Option<i32>>> {
        let edges = self.vertices
            .iter()
            .map(|v| v.edge
                .iter()
                .map(|e| e.to_option())
                .collect())
            .collect();
        edges
    }

    /// Return the vertices count in the graph
    pub fn amount(&self) -> usize {
        self.vertices.len()
    }
    pub fn indegree(&self, v: usize) -> Option<usize> {
        let v = self.vertices.get(v)?;
        Some(v.indegree)
    }
    pub fn outdegree(&self, v: usize) -> Option<usize> {
        let v = self.vertices.get(v)?;
        Some(v.outdegree)
    }
}
