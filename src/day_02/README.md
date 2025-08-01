# Day 02 - Safety Sequences

## General solution

We just need to count how many sequences satisfy some constraint and the safety constraint function (`SCF`) is the same in part 1 and 2, but in part 2 we are allowed to operate with gaps.

Prefix sums - but not sums! Prefixing and suffixing. Idea is we will compute is prefix/suffix of the sequence safe. We use powerful enums of Rust to do this (see in code `level_sequence_status.rs` for enum definition):
- `NoData` indicates lack of sequence data. This is when we look at prefix of the `sequence[0..0)` (no numbers)
- `Unsafe` indicates there has already been a safety violation in the sequence
- `Single(n)` indicates a sequence is safe and also explicitly that it is one number long, the number being `n`, used for prefix `sequence[0..1)`
- `Trend(n, b)` indicates a seqeuence is safe, ending with number `n` and boolean `b` tells what the trend is: `true` for increasing sequence and `false` for decreasing.

This prefixing can be calculated by starting from start of the seqeuence (`sequence[0..0)`) and adding one number at a time to calculate next enum based on the previous. Logic is a bit full of ifs and matches, but intuitively. See method `forward_analysis` in `level_report_analyser.rs`. I think it is fair to say it is dynamic programming based.

For example, sequence `[1, 2, 3]` will result in prefixed statuses:

`[NoData, Single(1), Trend(2, true), Trend(3, true)]`

Another example, `[1, 5, 6, 8]` will turn into:

`[NoData, Single(1), Unsafe, Unsafe, Unsafe]`

Because after a `Single(1)` the next number is 5, an absolute jump of 4 outisde of thresholds, making it unsafe. Adding any further number - still unsafe.

Okay, that's prefix. I mentioned suffix as well. Since I do not like code repetition: we reverse the actual sequence, calculate safety-prefixes of the reversed sequence and do some adjustments (see `backward_analysis` method).

## Part 1

Okay, now we have those safety-prefixes and safety-suffixes. First task, is the whole sequence safe? Just look at prefix/suffix that corresponds to the whole sequence and check the enum is all but `Unsafe`.

## Part 2

Okay, so now we can drop one number from the sequence and if the resulting reduced-sequence is safe, we say that such sequence passes the criteria.

Say we decide to drop number `sequence[i]`, the reduced sequence will be `safe-prefix[i]` (enum status of `sequence[0..i)` not including i-th) plus `safe-suffix[i+1]` (enum status of `sequence[i+1..]`). If those two prefix and suffix enums are "compatible" (see code), the sequence becomes safe by dropping i-th element.