use std::collections::{HashMap, HashSet};

use crate::day_23::model::{Connection, LocalNetwork};


mod error {
    const PREFIX: &str = "[D-23 graph]";

    pub fn construction(self_cycle_node: String) -> String {
        format!("{} failed to construct graph from provided local network. Self cycle node detected '{}'",
            PREFIX, self_cycle_node)
    }
}

pub struct LocalNetworkGraph {
    graph: HashMap<String, HashSet<String>>
}

impl LocalNetworkGraph {
    pub fn new(local_network: LocalNetwork) -> Result<LocalNetworkGraph, String> {
        let mut graph = HashMap::new();
        for Connection(left, right) in { let LocalNetwork(network) = local_network; network } {
            // if self-cycle detected, return error
            if left == right { return Err(error::construction(left)); }

            if !graph.contains_key(&left) { graph.insert(left.clone(), HashSet::new()); }
            graph.get_mut(&left).unwrap().insert(right.clone());

            if !graph.contains_key(&right) { graph.insert(right.clone(), HashSet::new()); }
            graph.get_mut(&right).unwrap().insert(left);
        }
        Ok(LocalNetworkGraph { graph })
    }

    pub fn nodes(&self) -> Vec<String> {
        self.graph.keys().map(|node|node.clone()).collect()
    }

    pub fn edges_from(&self, node: &String) -> HashSet<String> {
        self.graph.get(node).map(|edges|edges.clone()).unwrap_or_else(||HashSet::new())
    }

    pub fn edge_exists(&self, from: &String, to: &String) -> bool {
        self.graph.get(from).map(|edges|edges.contains(to)).unwrap_or(false)
    }
}