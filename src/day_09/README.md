# Day 09 - Disk Compacting

## Common logic

Again we have a disk compacter (`disk_compacter.rs`) that has a different compacter implementation depending on the part 1 vs part 2.

The compacter takes the input which is a sequence of numbers, and transforms it into `Vec<MemoryBlock>` (see `memory_block.rs`). This is the input to the `Compact` implementation. After the `compact` is invoked by the specific-compacter implementation check-sum is calculated using `try_fold` and all the safety stuff to account for potential overflows.

## Part 1

Use `block_by_block_compacter.rs` implementation. This one just keeps track of `left` and `right` index-pointers.
`left` sits at first free spot, `right` sits and last occupied block. We put occupied block at first free spot, and then update `left` by incrementing until next free spot and `right` by decrementing until next occupied spot.

Once `left` and `right` meet / pass one another (i.e. this becomes true `rigt < left`), we are done.

## Part 2

We use `file_by_file_compacter.rs` implementation.

This one uses segment tree (because it is faster and more complicated). So what does this segment tree contains?
It is constructed over a list (vector) of pairs representing empty spaces of form `(p, s)` where `p` represents index position in the memory space where a gap / empty space starts and stretches over a length of `s`.

E.g. say we worked out empty spaces to be `[(3, 5), (15, 3), (30, 7)]`, this means:
- At index 3 an empty space of length 5 is located
- At index 15 an empty space of length 3 is located
- At index 30 an empty  space of length 7 is located.

This also tells us where the "busy" spaces are, so in this example, there's a busy space at start at index 0 and is of length 3 (so [0, 1, 2] indices are occupied). Further, next busy space is at index 8 and it keeps being busy until and including index 14. And so on...

So what do we do with segmen tree? We create a custom function to join nodes into segmenet nodes starting from leaves. And how do we join them? Let's see what we want to query first.

Given a range of empty blocks `[e_0, e_1, ... , e_m]` we want to know what is the longest stretch of empty space that is the earliest. What does that mean? So we might have a few empty stretches of emtpy space of say length 10, but we would return the one whose index is the lowest.

That way, we kind of do binary search to work out for a given file of size `L`, we need at least empty space of length `L`, but if there's an earlier empty space of greater length, say `L + 3`, we would take it based on the criteria that we want to move it as left as possible.

If we find out that we do want to relocate the file, we have to update the empty spaces in the segment tree to reflect the change in empty spaces for future queries to be correct. Luckily, we do not alter the number of total empty spaces we have in the segment tree, we just edit a single empty-space parameters at a time and let segment tree do the recalc of the affected non-leaf segment nodes

So let's say we have an empty space at position 3 with length 8 and we put a file of length 5 there. This empty space is still going to be at the same index w.r.t. other empty spaces, only its length and absolute position will change within its boundaries. So after putting a file of 5 at "leftmost" (as closer to position 0) positions[3,4,5,6,7] get occupied, so the new empty space is created in its liueu at position 8, length 3.

Ok now that the segment tree usage is hopefully somewhat explained, what do we do?

Again, we have start from the end (the highest position). We take that file and its size, we use segment tree in a binary search way trying to locate the leftmost spot where the file would fit.

- We find out that it does not fit anywhere: move on.

- We find it fits at empty space `E`: we adjust `E` creating `E'` by reducing its empty-length and moving its start position by the length of the file. Update corresponding leaf-node in the segment tree, thus updating all relevant segment non-leaf nodes in the process. Move on.

Next file, repeat until we reach and finish processing the first file.

At the end, work out where is each file (moved and non-moved) and return the ordered list of occuppied memory blocks.

## Complexity analysis - part 2

Now, the actual solution I went with uses segment tree. Say the input is length of `N` (in this case, `N` refers to the number of digits we read from input file). To construct a segment tree we need something like `O(N log N)`.
Single query/modify of segment tree is `O(log N)`. For each file we kind of do binary search which is `O(log N)`.

So moving all files takes about `O(N * (log N)^2)` (`^` is power, not XOR). Finally, we do sort so that we get rearranged memory blocks in order w.r.t. position they start, so that's another `O(N log N)`. In conclusion, this is good.

## Alternative solution

Alternatively, we can use the fact that our input are digits: 0-9. Which means, empty spaces will be of length 0-9. Which means, we don't need a segment tree, we can just like use 10 "registers" of empty spaces (each register for exact length of empty space), for a file we wish to move, we can query the registers that contain sufficient length, plus we can organise those registers in some orderly manner (e.g. we can organise each register to be a binary-heap structure or something that optimises on something). Options are "endless" (not really), but the point is, the fact that empty spaces of interest (the ones left of the file we currently wish to move) will only be of lengths 0-9 and that is something we can use.

Nonetheless, that's an idea, I wanted to get a bit of practice with segment tree and trying to use some existing package and work out what things I need to do to make it work in Rust.

## Time

Part 1: 86 ms

Part 2: 95 ms