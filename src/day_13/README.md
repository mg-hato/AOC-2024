# Day 13 - Claw Machine Maddness

## Solution

Literally same for both parts but we "tweak" values for part 2 i.e. part 1 take original values, part 2 for each value in input we "tweak" it by adding like 10 million or billion or whatever (looks to be trillion? like four sets of three-0s).

Either way, yeah we are being asked effectively for each claw machine to like solve a system of equations.

Like, we have two buttons, button one with a single press modifies the claw position by adding X(1) and Y(1) to X and Y coordinates, respectively. Button 2 - same, X(2) and Y(2) movements.

Now say we pressed button 1 and 2, N and M times, respectively, we have two equations to try to solve:
- `X_goal = N * X(1) + M * X(2)`
- `Y_goal = N * Y(1) + M * Y(2)`

Where all the values are known, except for `N` and `M`. Just that. There's a bit of logic to ensure that no funky stuff takes place (e.g. division by zero, infinitely many solutions etc), but in a nutshell the code just solves this specific two variable two equation system of equations.

## Time

Part 1: 90 ms

Part 2: 92 ms