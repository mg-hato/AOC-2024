# Day 19 - Towels

## Solution

We calculate part 2 immediately and reuse it for part 1.

Base case: towel arrangement of length 0 is achievable in 1 way - by selecting no towel package.

Induction case: for towel arrangement `A` of length `k`, take all sub-arrangements `for i in [0..k): A[i..k)`.
For a given sub-arrangement, check if it part of the available towel packages. If it isn't, skip it. Otherwise, see in how many ways we could have arranged the sub-arrangement. Take the sum of those.

That's the idea, I implemented it by starting from the end and in the notation of suffixes. 0-suffix corresponds to the whole requested towel arrangement from the input.

Ultimately, we return a `Suffix -> Count` meaning that suffix `s` can be created in the given count of ways.
If the suffix has a mapping, that means it is achievable to be constructed. We pass this result to part 1 vs part 2 specific implementation to extract appropriate result.

## Part 1

We just check whether 0-suffix is achievable or not, and treat it as 1 or 0, respectively.

## Part 2

We take the associated value from the `Suffix -> Count` mapping for 0-suffix.

## Time

Part 1: 107 ms

Part 2: 117 ms