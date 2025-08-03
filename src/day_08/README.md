# Day 08 - Antinodes

## General solution

We have antinode counter (`antinode_counter.rs`). This one creates a frequency mapping `FM: Frequency -> Vec<UPosition>` such that for a frequency `f`, `FM(f)` is a list of all positions where anntenas of frequency `f` are located on the map. For all unordered pairs of two distinct positions with antennas of the same frequency, we run antinode calculator (trait def `antinode_calculator.rs`) to calculate where the antinodes are produced by the pair.

Part 1 and 2 differ in the implementation of the antinode calculator.

## Part 1

We use simple antinode calculator: `simple_antinode_calculator.rs`.

## Part 2

We use the other one: `resonant_harmonics_antinode_calculator.rs`.

The logic of both are just a bit of application of `UPosition` with `Movement` to achieve described antinode calculator behaviour.

## Time

Part 1: 71 ms

Part 2: 84 ms