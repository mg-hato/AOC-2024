use std::collections::{HashMap, HashSet};

use crate::{answer::Answer, day_05::{models::{PageOrderingRule, UpdatePages}, updates_checker::UpdatesChecker}, helper::result::collect, solver::Solve};

use super::models::RulesWithUpdates;


mod error {
    use crate::{day_05::models::UpdatePages, helper::display::vector_display};

    const PREFIX: &str = "[Solver D-05 P2]";

    pub fn multiple_correct_orders_error(update: &UpdatePages, num_1: u32, num_2: u32) -> String {
        let UpdatePages(pages) = update;
        vector_display(&vec![
            format!("{} multiple correct orders discovered.", PREFIX),
            format!("For example, numbers {} and {} do not have established order between themselves.", num_1, num_2),
            format!("Concretely, the update in question is [{}]", vector_display(pages, ",")),
        ], " ")
    }

    pub fn topological_sort_cycle_detection(update: &UpdatePages, resolved_size: usize, graph_size: usize) -> String {
        let UpdatePages(pages) = update;
        vector_display(&vec![
            format!("{} while running topological sort to establish the correct order,", PREFIX),
            format!("a cycle in the graph was detected due to incomplete dequeueing."),
            format!("Successfully popped from queue {} nodes, whereas graph has {} nodes.", resolved_size, graph_size),
            format!("Concretely, the update in question is [{}]", vector_display(pages, ",")),
        ], " ")
    } 
}

pub struct UpdatesCorrector;


struct Node {
    successors: HashSet<u32>,
}

impl Node {
    pub fn new() -> Node { Node { successors: HashSet::new() } }

    pub fn add_edge(&mut self, id: u32) {
        self.successors.insert(id);
    }
}

impl UpdatesCorrector {
    pub fn new() -> UpdatesCorrector { UpdatesCorrector }

    /// Create DAG (Directed Acyclic Graph).
    /// A page number represents a node.
    /// A `PageOrderingRule` represents an edge; for rule `x|y` that corresponds to edge
    /// going from node `x` to node `y`.
    fn create_dag(rules: &Vec<PageOrderingRule>, update: &UpdatePages) -> HashMap<u32, Node> {
        let mut graph = HashMap::new();
        let UpdatePages(pages) = update;
        pages.iter().for_each(|&page|{ graph.insert(page, Node::new()); });

        for PageOrderingRule(from, to) in rules.iter() {
            if graph.contains_key(from) && graph.contains_key(to) {
                graph.get_mut(from).unwrap().add_edge(*to);
            }
        }
        graph
    }

    /// Returns a hash map that contains mappings of form X -> C, meaning,
    /// a node X has C incoming edges
    fn get_dependency_count(graph: &HashMap<u32, Node>) -> HashMap<u32, i32> {
        let mut dependency_count = HashMap::new();
        graph.values().flat_map(|node|node.successors.iter()).for_each(|&id|{
            let count = *dependency_count.get(&id).unwrap_or(&0);
            dependency_count.insert(id, count + 1);
        });
        dependency_count
    }

    /// Runs a topological sort on the DAG. Nodes for the DAG are the pages used in the provided update.
    /// Edges are only the relevant page ordering rules that affect the pages in the provided update.
    /// This function can result in failure (e.g. a cycle exists in the graph or multiple valid re-orderings exist).
    /// If it determines that there is a unique correct re-ordering of the pages, it will return it.
    fn determine_order(rules: &Vec<PageOrderingRule>, update: &UpdatePages) -> Result<UpdatePages, String> {
        let graph = Self::create_dag(rules, update);
        let mut dependency_count = Self::get_dependency_count(&graph);

        // form a queue of next to pop. we start with nodes whose dependency count is zero   
        let mut queue = graph.keys()
            .filter(|id|*dependency_count.get(id).unwrap_or(&0) == 0)
            .map(|&id|id)
            .collect::<Vec<_>>();

        let mut order = vec![];

        // we expect that at all time we will only have one node to pop. If that is the case,
        // that means that there is only one solution to topological sort on the given graph
        while queue.len() == 1 {

            let id = queue.pop().unwrap();
            order.push(id);

            // update dependency count on all successors of the current node
            for successor in graph.get(&id).unwrap().successors.iter() {
                let updated_count = *dependency_count.get(successor).unwrap() - 1;
                dependency_count.insert(*successor, updated_count);
                if updated_count == 0 {
                    queue.push(*successor);
                }
            }
        }
        
        // Now that we are out of the loop, determine whether order was successfully resolved
        if queue.len() > 1 {
            Err(error::multiple_correct_orders_error(update, queue[0], queue[1]))
        } else if order.len() != graph.len() {
            Err(error::topological_sort_cycle_detection(update, order.len(), graph.len()))
        } else {
            Ok(UpdatePages(order))
        }
    }
}

impl Solve<RulesWithUpdates> for UpdatesCorrector {
    fn solve(&self, input: RulesWithUpdates) -> Result<Answer, String> {
        collect(UpdatesChecker::get_updates(&input, false).iter()
            .map(|&update|Self::determine_order(&input.rules, update)).collect())
            .map(|updates|RulesWithUpdates { updates, ..input })
            .and_then(|corrected|UpdatesChecker::new().solve(corrected))
    }
}