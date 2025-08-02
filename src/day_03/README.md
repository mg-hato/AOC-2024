# Day 03

## General solution

Using RE we extract in order all instructions that are recognisable:
- `Do`
- `Don't`
- `Mul` - concretely parameterised enum `Mul(x, y)` with `x` and `y` being non-negative integer numbers.

After that we process each instruction in a fold-like manner (concretely in Rust: `try_fold`), starting with the accumulator: `(0, false)`

The accumulator carries the following information in that pair, call it `(n, b)`:
 - `n` being the accumulated sum of `Mul`-products so far
 - `b` being boolean that tells whether to ignore next `Mul` instruction. The next `Mul` instruction is skipped / not accounted for if and only if `b` is `true` (`b` is called `ignore` in code).

 ## Part 1

 Some extra logic to ensure that the accumulator will always have the second element set to `false`. This implies that all extracted `Mul` instructions will be accounted for.

 ## Part 2

 Here, we "toggle on" the ignoring logic, so that `Do` and `Don't` instructions affect the `ignore`/`b` boolean of the accumulator.

 Basically the mul-processing logic with or without conditional instruction recognition is in method `process_instruction` and the only diff between the two parts is the boolean value of `MulExtractor` called `conditional_detection` being assigned differently by "constructor" call (i.e. Rust, `new` method). The rest falls into pieces (probably)?