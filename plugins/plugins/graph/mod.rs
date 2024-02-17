use std::path::PathBuf;

use petgraph::graph::{DiGraph, NodeIndex};

struct PlumGraph<N, E> {
    graph: DiGraph<N, E>,
    path: PathBuf,
}

impl<N, E> PlumGraph<N, E> {
    pub fn new(write_path: String) -> Self {
        PlumGraph {
            graph: DiGraph::new(),
            path: PathBuf::from(write_path),
        }
    }
}
