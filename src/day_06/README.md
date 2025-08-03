# Day 06 - Guardian of the lab

## Part 1

Use `UPosition` and `Movement` from shared code plus extra logic to simulate guardian's path.
As `UPosition` implements hashing and equality check (thanks to Rust's `[derive(*)]` functionality), put all positions visited by the guard in a set to ensure distinct-ness. Count of that.

## Part 2

Use part 1 to get all positions that the guard visits. Those are prospective candidates for an obstacle. Reasoning: if the guard does not visit position `P`, putting an obstacle there won't make a difference.

For each of candidate obstacle positions, call it `OP`, kind of make the copy of the map and make `OP` position to be blocked and run the logic from part 1.

Simulating the guard's patrol from part 1 actually does more than I initially described. Commong logic is in `map_analyser.rs` in method `perform_analysis`. This returns `Result<Vec<GuardState>, String>`:
 - `Err(message)`: there's an error, nothing to be done, just propagated to the output.
 - `Ok(states)`: if the `states` are empty, this means that the guard loops. O/w it is not empty and contains a sequence of guard's states (i.e. positions and directions the guards faces in order) until they leave the observable space. In part 1, if the guard loops, that's considered an error. In part 2, if the guard loops on the original map, it is also an error, but if they loop on the "modified" map with one added block, that means the newly blocked position `OP` is a valid blocking position candidate. In which case, we count it.

And that's all is to it, we simulate guard's patrol for every candidate blocking position and get feedback whether it loops or not. We count on how many it loops.

## Part 2 - optimisation

Now, there's actually a "small" optimisation that takes place in simulation process of part 2 to speed up the simulation. To be honest, it might be very unnecessary. So let's take it step by step.

Originally, simulating the guard's patrol was done step by step, where the guard moves one unit movement at a time.

Knowing that we will run multiple simulations on the same map with minimal edits, how can we speed this up?

We "cache" multiple steps in something called a "leap". So if the guard is at position `P`, they will move in the same direction until they hit a first obstacle (or leave the map). Merging all these steps into one, we get a leap. The guard was a position `P` and then they leaped out of the map or they leaped onto the next obstacle changing the direction they face.

So for example, instead of having 10 simulation steps where guard was just going straight until at 10th step they hit obstacle and changed direction, we get one merged leap. We just cache that.

Adding one obstacle? Again, most of the map will have the same leap-cache, only special care needs to be taken when the guard is in row or column where the added block is to work out whether we can use the leap-cache or whether they will hit the newly added obstacle.

See `optimised_caching_loop_detector.rs` for the idea of leaps with no obstacles. After, see `adjusted_loop_detector.rs` to see how that one obstacle changes this whole caching logic etc.

## Time

Part 1: 107 ms

Part 2: 193 ms