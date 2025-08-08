# Day 14 - Rumbas Borderline

## Solution

Take a robot. Interested where it will be in like, ... `N` iterations where we know theirs movement per iteration: modulo arithmetic. Figure out after whatever steps where they are and etc.

Part 1: We just do that for 100 iterations

Part 2: This is a bit funky. Since we expect a tree and the cleaning field is of sizw 101 by 103, that means that after (101 * 103) iterations we will be at the same place (modulo arithmetic maths). So we generate all those iterations from 0, till mathematically guaranteed first repetition of cleaning-room state, format it in a map and put it in the output (which I piped into a file and did ctrl+F on consecutive 4-robots on the map).

## Time

Part 1: 80 ms

Part 2.

- Running CMD version where input is not saved into a file: 14.4 seconds

- It takes 40 seconds when all is done + writing to a file (probs massive overhead in disk writing)