# Day 05 - Page printing

## Part 1

Take update page orders and create a set. If there's a page ordering rule `p|q` then set contains an ordered pair `(p, q)`. Call this set `R` - for Rule-set

Go through proposed page printing orders and check for each whether it violates page ordering rules. For sequence currently inspected, call it `P`, take all possible pairs `(i, j)` where `0 <= i < j < length(P)`. Check that pair `(P[i], P[j])` is not in the set `R`.

For correctly ordered page printing sequences, work out middle and do safe-sum.

## Part 2

Re-use the logic from part 1 to work out which sequences are incorrectly ordered.

For incorrectly ordered sequence `P` create a Directed Acyclic Graph (DAG) as follows:
- DAG nodes are pages from the sequence `P`
- DAG edges are page ordering rules of interest. Say there's page ordering rule `p|q`. This rule is of interest if and only both `p` and `q` are nodes of the DAG. In that case, add edge from `p` to `q`.

Run DAG topological sort, ensure there's only one correct ordering (otherwise what's the middle page?) and do whatever else is required to solve the problem - probably safe-sum of middle pages of correctly ordered sequences.

## Time

Part 1: 90 ms

Part 2: 91 ms