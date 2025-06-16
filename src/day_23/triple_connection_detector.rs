use crate::{answer::DisplayableAnswer, day_23::{local_network_graph::LocalNetworkGraph, mesh_finder, model::LocalNetwork}, solver::Solve};

pub struct TripleConnectionDetector;

impl TripleConnectionDetector {
    fn count_chief_historian_candidates(tri_connections: Vec<Vec<String>>) -> usize {
        tri_connections.into_iter()
            .filter(|tri|tri.iter().any(|computer|computer.starts_with("t")))
            .count()
    }
}

impl Solve<LocalNetwork> for TripleConnectionDetector {
    fn solve(&self, input: LocalNetwork) -> Result<crate::answer::Answer, String> {
        LocalNetworkGraph::new(input)
            .map(|network|mesh_finder::find_meshes_of_size(&network, 3))
            .map(Self::count_chief_historian_candidates)
            .map(DisplayableAnswer::new)
    }
}