# Day 16 - Reindeer Maze

## Common Logic

So we have a maze and a reindeer who is initially at position `p` facing direction `d`. Thus, let's define a state of the maze race:
- Reindeer's position
- Reindeer's direction

We are allowed, three transformations on the state:
- Turn left
- Turn right
- Go forward (if possible)

Imagine the states are nodes of the graph and these transformations are edges of the graph. Further, on each edge we have a weight/distance (associated cost). At this point, I will say, Djikstra's algorithm.

Once the algorithm is run, we return a mapping `State -> MinScore` for all reachable states. We pass that mapping to specific implementation of analyser (see `reindeer_path_analyser.rs`), that is dependant on part 1 vs 2.

## Part 1

We work out what the end position is, and we find the state with lowest score whose position matches the end position (that's because we might have multiple states - concretely 4 - at the end position but with only differing facing directions - as we only care that the reindeer reached the goal and not so much what direction they are facing, we opt in for the lowest score).

## Part 2

Again, we grab the best scores at finish line. We define logic to undo moves. For each optimal path state, we do the undo and see whether we have the undo-states in our result from Djikstra's algorithm mappings, if we do, it means reindeer can potentially use that path as an optimal path, in which case we record such state as eligible. We keep on doing this in a queue-like fashion until we reach the start state.

For all states that are in the "optimal path" we record how many distinct position that covers, returning it as the answer.

## Time

Part 1: 127 ms

Part 2: 135 ms