use std::collections::HashSet;
use crate::day_23::local_network_graph::LocalNetworkGraph;


/// A mesh in a graph is a set of nodes where any two nodes are directly connected.
/// A mesh is represented here as an alphabetically sorted vector of nodes (strings).

/// Given local network graph and same-sized meshes of size `n` returns a sequence
/// of meshes of size `n + 1`
fn get_incremental_meshes(network: &LocalNetworkGraph, meshes: &Vec<Vec<String>>) -> Vec<Vec<String>> {
    let mut participants = HashSet::new();
    for mesh in meshes.iter() {
        for participant in mesh.iter() {
            participants.insert(participant.clone());
        }
    }

    let mut next_meshes = Vec::new();
    for participant in participants {
        for mesh in meshes.iter() {
            let last = &mesh[mesh.len() - 1];
            if last.lt(&participant) && mesh.iter().all(|node|network.edge_exists(node, &participant)) {
                next_meshes.push(vec![mesh.clone(), vec![participant.clone()]].concat());
            }
        }
    }
    next_meshes
}

/// Creates a sequence of 'edge meshes' i.e. all meshes of size 2.
fn get_edge_mesh(network: &LocalNetworkGraph) -> Vec<Vec<String>> {
    let mut edge_meshes = vec![];
    for node in network.nodes() {
        for adjacent_node in network.edges_from(&node) {
            if node < adjacent_node {
                edge_meshes.push(vec![node.clone(), adjacent_node.clone()]);
            }
        }
    }
    edge_meshes
}

/// Returns all meshes of given `size`. For requested size less than 2 an empty sequence of meshes is returned.
pub fn find_meshes_of_size(network: &LocalNetworkGraph, size: usize) -> Vec<Vec<String>> {
    if size < 2 { return vec![]; }
    let mut meshes = get_edge_mesh(network);
    for _ in 2..size { meshes = get_incremental_meshes(network, &meshes); }
    meshes
}

/// Returns a sequence of largest meshes existing in the network.
pub fn find_largest_meshes(network: &LocalNetworkGraph) -> Vec<Vec<String>> {
    let mut meshes = get_edge_mesh(network);
    loop {
        let next_meshes = get_incremental_meshes(network, &meshes);
        // if the next meshes sequence is empty, it means we have reached the limit
        if next_meshes.len() == 0 {
            return meshes;
        }
        meshes = next_meshes;
    }
}