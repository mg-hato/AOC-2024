# Day 21 - Keypad Indirections

## General Solution

This one has an interesting solution and I really enjoyed thinking about it.

The main idea is caching.

Say you are commanding `N` robots using the task-described indirection, robots: 1, 2, ... `N`.

Observe that for a robot `i` telling it to press a button `X` all other robots `j` with `j < i` they will all be pointing to a button `A` (accept/enter/press) and in a sense, they will reset.

Say now that we have cached for robot `i` how many button presses you need in order to tell him to move by a movement `M(x, y)` (to change the position of their finger pointing from current row by `x` and current column by `y`) and then press a button, making all their predecessors pointing to `A` button again, reseting their predecessor states.

Now we can use that cache to make a next cache for robot `i + 1`: starting from some position, move by some `M'` movement and then press newly pointed button.

Concretely, the cache will have ordered movements, so for movement `M(x, y)` those would be actually be decomposed to either `RowThenCol(x, y)` or `ColThenRow(x, y)` (because sometimes we need to be mindful of the gaps that the robot cannot hover over with their digit).

Once the cache for the final / penultimate robot is created (I forgot my implementation detail), we want that robot to press say `123A`. We work out viable movements from:
- Initially pointed `A` goes to `1`. Work out viable ordered movements, and if there are multiple such movements, choose the cheaper w.r.t. number of button presses.
- Then from `1` goes to `2` and a press. Do the same as above.
- Repetition.

The solution is parameterised on the constructor on how many robots are using directional keypads to control the last robot using the numeric keypad.

## Part 1

The indirectional chain length is 2 (meaning the human user using directional keypad, controlling 2 robots using directional keypads and a 3rd robot controlling the numeric keypad).

## Part 2

The indirectional chain lengt is 25 (human, 25 robots directional keypad, 26th robot numeric keypad)

## Time

Part 1: 94 ms

Part 2: 120 ms
