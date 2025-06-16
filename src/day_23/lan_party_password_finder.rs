
use crate::{answer::DisplayableAnswer, day_23::{local_network_graph::LocalNetworkGraph, mesh_finder, model::LocalNetwork}, helper::display::vector_display, solver::Solve};


pub struct LanPartyPasswordFinder;

mod error {
    const PREFIX: &str = "[D-23 solver part 2]";
    pub fn find_lan_party() -> String {
        format!("{} LAN party could not be found. There's either multiple candidates or zero.", PREFIX)
    }
}

impl LanPartyPasswordFinder {
    /// Verifies the sequence of largest meshes has exactly one element, as LAN party must be unique
    fn get_lan_party(largest_meshes: Vec<Vec<String>>) -> Result<Vec<String>, String> {
        if largest_meshes.len() == 1 {
            Ok(largest_meshes[0].clone())
        } else {
            Err(error::find_lan_party())
        }
    }
}

impl Solve<LocalNetwork> for LanPartyPasswordFinder {
    fn solve(&self, input: LocalNetwork) -> Result<crate::answer::Answer, String> {
        LocalNetworkGraph::new(input)
            .map(|network|mesh_finder::find_largest_meshes(&network))
            .and_then(Self::get_lan_party)
            .map(|lan_party|vector_display(&lan_party, ","))
            .map(DisplayableAnswer::new)
    }
}