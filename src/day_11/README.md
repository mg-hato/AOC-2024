# Day 11 - Plutonian Stones

## Solution

We transform our input into a mapping: `S -> C` where `S` is a stone number and `C` is the count, so if the mapping contains `5 -> 10` that means there are 10 stones with number 5.

After that, we produce the `next` method to get the next state after a blink. Which does the logic described in the problem statement and returns a next state in the same mapping format. Thus for each `next` invocation, we effectively process only stones that have a non-zero count exactly once, using their count to avoid stone-by-stone expansion (that is probably the trick, because we get a lot of stones in total very quickly).

We apply the `next` function as many times as needed (this is part-dependant). Concretely, 25 and 75 times for part 1 and 2, respectively.

## Time

Part 1: 75 ms

Part 2: 107 ms

The only difference between the parts as outlined above is how many applications of `next` method we use. And to illustrate the growth-rate, for 25 and 75 invocations my tailored input produces as a solution a 6- and a 15-digit number, respectively.