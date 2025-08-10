# Day 14 - Rumbas Borderline

## Solution

So we are interested where the robot will be given their initial coordinates and their movement-vector per unit of time. Because they are bound to remain in the enclosed rectangular space due to phasing/looping onto the other end once they were to leave the rectangular boundaries, we can easily and mathematically calculate their position after `N` units of time using simple addition, multiplication and modulo arithmetic.

For X-coordinate, given `x0` (their initial X-coordinate position), `dx` (their movement in X-coordinate, it can be negative), `b` (bounary i.e. the number of rows: row 0, row 1, ..., row `b - 1`) and `N` (units of time since start), the position at unit time `N` is `xN = (x0 + dx * N) mod b` s.t. `0 <= xN < b`. Similar idea for Y-coordinate etc.

Part 1: We just do that for 100 iterations for each robot and work out whatever we need to (something with 4 quadrants etc).

Part 2: This is a bit funky. Since we expect a tree and the cleaning field is of size 101 by 103, that means that after (101 * 103) iterations we will be at the same place as we have been initially (modulo arithmetic, proof is left as an exercise for the reader). So we generate snapshots of the cleaning field for each unit time from 0 until 101 * 103 (not including the right bound).

We then manually analyse each snapshot to see if something forms the shape of a tree. Or alternatively, just do ctrl+F on "****" and there might be three or so snapshots that will be found to have the pattern, one of them most likely being with a christmas-tree pattern.

## Time

Part 1: 80 ms

Part 2:

- Running CMD version where input is not saved into a file (i.e. goes on command line output): 14.4 seconds

- If we write the output to the file, it takes 40 seconds when all is done (probably massive overhead in disk writing)

- Lastly opening the file and hoping that the text editor of choice will not crash and doing ctrl+F on "****" takes some additional seconds.