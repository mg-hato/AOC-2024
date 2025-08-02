# Day 04 - XMAS Wordsearch

## General solution

Nothing fancy, just a word search with word search rules. However, this can be a bit annoying since we need to somehow handle various directions (up, left, up-left diagonally, etc.).

I believe this is the point in AOC 2024 when the first use of `UPosition` and `Movement` comes into place.

`UPosition` is unsigned coordinate position with the notion of row-column coordinates. `Movement` is an idea of change in `UPosition`, mathematically kind of like vector (mathematical one, not Rust/C++ vector).

With this we see where we are going. So to investigate say down-left diagonally, we start at position `P` and we keep applying unit movement `Movement{ row: +1, column: -1 }` and check letters as we go.

## Part 1

We do just that above described. Take each position in the word search table and for each unit direction (8 of them) we check for word `XMAS`.

## Part 2

We need to find `cross-MAS` occurrences (explanation on AOC 2024 website).

Take each position in the word search, check it is letter 'A'. Thus, it is a candidate to be the "center" of a cross-MAS. Check each diagonal forms letters 'M' and 'S' on the edges. If so, that's a cross-MAS. 

## Time

Part 1: 90 milliseconds

Part 2: 94 milliseconds