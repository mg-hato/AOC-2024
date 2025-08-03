# Day 07 - Calibration Equations

## General solution

Starting with a full equation. Say `18: 3 5 3` we try to reduce it if possible into prospective path-equation that has fewer right hand side elements. We know that this whole equation will be resolved something like `((3 op1 5) op2 3` where `op1` and `op2` are some operations. So we start working backwards, making assumptions on `op2`:

- Say `op2` is `+`. Then `18 = X + 3` where `X = 3 op1 5`. If `op2` is indeed `+` we can derive sub-equation that might or might not lead to a solution, working out unknown left hand side `X` we get it to be 15. The next sub-equation is `15: 3 5`. If we can solve this, we can solve the "parent". And concretely we would do `op1` being `*` and reduce it again using same approach of `15 = X2 * 5` where `X2 = 3`, getting simple equation `3: 3` which is `3 = 3` which is true. Hence, this whole "sub-tree equation path" leads to a solution, meaning the ultimate parent equation is solvable.

- Other alternative, which we might not consider given the above case solved successfully, is if `op2` is `*`, reduce it, get `6: 3 5`, which we reduce again to trivial equation `1: 3` which is `1 = 3` which is false, concluding this sub-path will not yield solution.

However, note that we also should kind of have checks whether a certain operation can be used to produce "reduction equation". Say we have `18: 5 6 7`. Here we cannot use rightmost operation to be `*` because 7 does not divide 18 without remainder. So we can only use addition, etc etc etc reductions, we get `18 = 5 + 6 + 7`.

So, we introduce abstraction of operation. See trait/interface in `operation.rs`. We give result and right hand side element and we try to derive left hand side element. This can succeed or not, so it returns `Option<*>`. We implement these and give them as valid operations to use in our reduction logic.

## Part 1

We implement `Addition` and `Multiplication` operations and pass them to reduction logic (see `calibration_results_checker.rs`). That solves it.

## Part 2

We implement `Concatenation` and pass it as a third option to the same reduction logic checker.

## Time

Part 1: 58 ms

Part 2: 58 ms