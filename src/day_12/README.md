# Day 12 - Fence Pricing

## Common Logic

When solving this problem, we need to work out what is the area of same-plant-type-island and where the fences are at. This is a common bit to both part 1 and 2. So, in the `fence_price_calculator.rs` we put the logic that works out the same-plant-type areas and "builds" a fence around them. A fence in this solution is defined as a pair of `(p, d)` where `p` is the inside position where the fence is and `d` is the direction where the fence is from the position `p` (see `fence_unit.rs`).

Basically what this is kind of like is "oh there's a fence at square-ish field at position (x,y), and the fence is - say - between (x-1,y) and (x,y), or alternatively said, there's a fence immediately north of position (x,y)". (because decrementing a row we go "up" / north etc). But that's the idea, we define the fence by a position + relative direction, and to standardise things we always look at the position that is "inside the fence" - because in this task, the fence is there to divide a same-plant-area from a some-other-field (can be outside-of-bounds field if looking at edges of input) - thus we define our fence as an immediate position belonging to the area that is being isolated at the moment + the immediate direction from the position where the fence is.

So that's idea, the code does that. There's ways to code it. Now that we work this out, we can also easily work out the set of all fields/positions that belong to currently analysed isolated area. At that point, we effectively know the area, and we also know all "fence units" that guard the area - that's the common bit - how we work out how many "fences" we have is a matter of part 1 vs part 2 implementation.

## Part 1

So, we have all the fence units (see above), in part 1, we literally care about how many of those units we have, so we just use the length/size of the fence-units hash-set provided.

## Part 2

Now we are kind of joining the fence units in the "continuous fences". To do this, we just do a bit of exploration from the current fence unit being analysed. E.g. say we have a fresh fence unit at position (5, 8) facing say north/up (so a fence between internal field (5, 8) and external field (4, 8)). Now we just wonder: is there a fence that is "adjacent" that we can join with it to form a continuous fence. Well, in this case there are two fences to consider: there's one at (5, 7) and (5, 9), both being north-facing. If any of the two fences is available, then we merge that fence and continue merging more of other unexplored fences along  this line. 

That's roughly the idea, for more details see `discounted_perimeter_calculator.rs`.

## Time

Part 1: 95 ms

Part 2: 100 ms