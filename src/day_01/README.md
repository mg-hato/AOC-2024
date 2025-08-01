# Day 01 - Number pairs

## Part 1

Nothing too fancy. Zip up left and right number, calculate the diff and do safe-sum.

## Part 2

Nothing too fancy, again. Make a frequency map on the second element: mapping of the form `X -> F` meaning that second element number `X` appears in total `F` times.

Use this frequency map to calculate similarity score for each first element number (if mapping is not present for the given number `X`, it means it appears 0 times). Do safe-sum.

## Time

Part 1: 87 milliseconds

Part 2: 83 milliseconds