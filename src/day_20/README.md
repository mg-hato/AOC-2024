# Day 20 - Race Track Cheating

## General Solution

We perform breadth-first-search from the end to work out how far away each position is from finish.

Then for each one of those positions we explore the limited range of movements that can be performed within the allowed cheat-time. For each such prospective cheat, we analyse how much time we actually save and put it in the list/filter cutoff (that's because we are interested in the cheats that only gain us at least some predefined speed-up).

The cheat time is parameterised in the respective constructor of `cheats_counter.rs`.

Once we have a list of distinct cheats that are gaining us at least some predefined time-gain, we do the rest of the processing to get the answer.

## Part 1

Cheat time is 2

## Part 2

Cheat time is 20

## Time

Part 1: 115 ms

Part 2: 400-450 ms

Part 2 time can potentially be optimised