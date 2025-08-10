# Day 18 - Corrupted Bytes

## Part 1

Overall its a labyrinht like problem. Use something like breadth-first-search BFS. Find the shortest path.

## Part 2

Reuse algorithm from part 1 and binary search to work out which byte will do a cutoff. That's because if byte `i` cuts off the path, the path remains cut off for bytes `i + 1`, `i + 2`, ...

Alternatively, do it iteratively.

## Time

Part 1: 88 ms

Part 2: 96 ms