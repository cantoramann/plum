use lazy_static::lazy_static;
use petgraph::graph::DiGraph;
use std::sync::{Arc, Mutex};

pub struct PlumGraph<N, E> {
    graph: DiGraph<N, E>,
}

impl<N, E> PlumGraph<N, E> {
    pub fn new() -> Self {
        PlumGraph {
            graph: DiGraph::new(),
        }
    }
}
