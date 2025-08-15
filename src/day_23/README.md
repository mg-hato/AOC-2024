# Day 23 - LAN Party

## General Solution

So the idea is to find fully connected meshes (FCMs) or subgraphs in the given graph where all the nodes in the subgraph are connected by an edge. We make iterative solution that will help us work for both part 1 and part 2.

Base case: We find all FCMs of length 2. I.e. all sets of two nodes that are connected by an edge. Easy to do.

Iterative case: Once we have FCMs of length `i` we want to find FCMs of length `i + 1`. Roughly how it works. We take an node `N` and all of the meshes of length `i`. Eliminate the ones where `N` already participates. For the rest, check that node `N` and all of the FCM participants have an edge connecting, thus being able to form an FCM of length `i + 1`.

FCMs are stored as an ordered vector (i.e. sorted by node's names). Further, for the sake of optimisation, say we have an FCM `[m1, m2, ... , mi]` and node `N`. We will only combine this FCM with the node `N` to form incrementally larger FCM if and only if `name(mi) < name(N)`. This is just an optimisation step to avoid too many checks and too many comparisons and potential candidates. Why?

Imagine that we can create an FCM of length `j = i + 1` that is going to be `G = [m1, m2, ... , mi, mj]`. That means that there are `j` many FCMs that we can create it from `G \ m1` (`G` without `m1`) with `m1`, then `G \ m2` with `m2`, ...., `G \ mj` with `mj`. With our iterative approach all these `j` many with corresponding missing node will create FCM `G`, but because our optimisation we will opt in for creating `G` combining alphabetically the greatest node `mj` with FCM `G \ mj`, eliminating other `j - 1` possibilities. This is just an optimisation and it speeds up a bit.

## Part 1

We use our iterative approach to get FCMs of length 3 and we eliminate the ones that do not start with letter `t` or something.

## Part 2

We loop incremental creation of a larger FCM until we get to the point that there's only one such FCM of the latest run lenght `k`. If there is one such FCM, our problem has a solution, and running the incremental FCM finding logic will give us that there's no FCM of length `k + 1`.

## Time

Part 1: 140 ms

Part 2: 3.3 seconds

Maybe some further optimisation can be found for part 2.