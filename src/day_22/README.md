# Day 22 - Monkey Trading

## Part 1

Just implement the required transform of the number and repeat it as many times required.

## Part 2

Use the part 1 to generate sequences of the prices. For each price-sequence, create a mapping `ChangeSequence -> Profit`.

To calculate that, for each position `i` in the sequence, extract the observed change sequence starting with `i` and ending with `i+c` where `c` is the length of change sequences that the monkey uses in part 2. Consideration to take into account is index-out-of-bounds issue, so extra defensive logic on that.

Once the change sequence starting from `i` is extracted, check that it is its first appearance. If so, register the profit if would yield for this price-sequence.

Since we have the mapping from change-sequence used to profit yielded for each price-sequence, the next step is to join sequences in the most logical way: take the sum for each change-sequence in two mappings to create a joined mapping (if one mapping does not contain the change-sequence as a key, take zero for it).

Once all mappings are joined up into one, iterate through all mappings and take the greatest profit recorded in the all-joined mapping.

## Time

Part 1: 101 ms

Part 2: 3.1 seconds

Note: Maybe part 2 can do with some optimisation.