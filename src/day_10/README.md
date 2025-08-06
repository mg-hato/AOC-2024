# Day 10 - Trailheads

## Common logic

So there's a lot of common logic in this one (see `trailhead_review_analyser.rs`).

It uses something like a depth first search (DFS), but slightly modified to allow exploration of the same node if it is reached by multiple differenet ways - this is safe to do because the graph / topological map and movements defined over it are also satisfying the DAG properties - concretely of interest, we can only go up as part of our allowed movements, and we will go as much as from height 0 to height 9.

We run this modified-DFS on all positions of height 0, while on others we short-circuit and return 0 as its rating/review. Once we run the algorithm from height 0, we "register" every reachable position at height 9 as many times as there are different paths leading to.

The "register" method is part of the trait `Review`, implementation of it is part-dependant.

## Part 1

We use `trailhead_score.rs` implementation. This one registers a position into a set. Thus effectively, it does not matter how many different ways we have to reach a position of height 9, it only accounts to how many different positions of height 9 we can actually reach.

## Part 2

We use `trailhead_rating.rs` implementation. This one is a counter. Every time we register a position to it, it increments the counter - therefore, here it matters how many times we have registered a position i.e. how many different ways to reach a position is accounted for by the modified-DFS implementation through number of invocations of `register` method.

## Time

Part 1: 80 ms

Part 2: 81 ms

Roughly both are using the same modified-DFS, and even the part 2 is "lighter" - it only keeps a track of a counter, whereas part 1 keeps a Hash Set to ensure distinctness.