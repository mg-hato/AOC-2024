# Advent of code 2024 - Rust adventure

## Intro

After finalising the Advent of Code (AOC) 2024 in approximately month July 2025, I have decided to document my journey. And I will use these README files mainly for that, documenting the journey, explaining ideas behind solutions and occasionally giving instructions on how to build/run the code.

This can be useful for any observers of this project as well as for myself once I completely forget about this project and how Rust works.

## Running the code

Mainly, install Rust, Cargo etc (I don't remember what I needed to install initially). Once done:

1. To run all tests: `cargo test`
2. To build unoptimised build: `cargo b`
3. To build fast optimised build: `cargo b --release`

Next step, run the thing on input file. For that, locate `*.exe` file. Usually for my usecase that would be (assuming release executable): `./target/release/aoc_2024.exe`

Once the executable is located, we can run the following two options:

1. `<exe_path> --f <input_file> --d <day_number>` to run solution for part one of the problem.

2. `<exe_path> --f <input_file> --d <day_number> --p2` to run solution for part two of the problem.

So for example, having an executable in `./target/release/aoc_2024.exe`, input file for day 11 at `./input.txt` and wanting to run against part two of the problem, the following would do:

`./target/release/aoc_2024.exe --f ./input.txt --d 11 --p2`

These are the basics, more functionality is supported by the command line arguments inference, but I will not go in details here.