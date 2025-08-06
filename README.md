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

## Structure

All logic for each day is located in its designated folder and copy-past file that declares all imports (e.g. day 15 is in `./src/day_15` and `./src/day_15.rs`).

Minor exception is shared code e.g. a lot of the time problems involve some idea of (x,y) coordinates system. This is captured in shared code `UPosition` (unsigned position) with a lot of methods that are quite handy for various cases.

Other example is that things/input can be of rectangular structure (e.g. a map with obstacles and free spaces, with N rows and each row of length M, effectively NxM table), that format of data is captured in shared code `Table`. And so on.

## Pipelines

Each day is defined as a "pipeline" or `pipeline_executer` rather. A pipeline consists of the following steps:

- Reader: reads the input file, trims comments, returns list of `Line` (text + original row number of the text). Usually similarish for all days. Can fail and return `Err(String)` explaining why it failed (e.g. given input filepath does not exist etc.)

- Parser: parses the text into input data. Day-dependant. Can fail and return some explaination of why it did, in form of `Err(String)`. Otherwise returns successfully parsed input data in form of `Ok(data)`.

- Verifier: looks at input data and says `Err(String)` if it sees something wrong with the data, otherwise `Ok(data)`. Generally, rarely used. Verifying logic was moved into next component, to perform kind of "Try-And-Do" logic corresponding of C# `TryParse` etc. Reason is that doing something with the data and failing shares a lot of the code with just verifying and then doing (if it makes sense).

- Solver: attempts to solve the problem. This is day-dependant and part-one-or-two dependant. Again, if it fails for some reason, returns `Err(String)` otherwise returns something called `Answer` (wrapped in success so: `Ok(Answer)`). `Answer` is just usually a wrapped for a piece of data that is being reported as the solution (e.g. `Ok(Answer(5))` to say that the answer is number 5). Occassionally the answer gets more complex, but the idea is that the `Answer` interface (trait in Rust) has a method `report()` that prints out the answer. When that reporting gets complex, it gets complex and will described in respective days. Most of the time, the actual implementation will be something called `DisplayableAnswer`.

And as such, the pipeline is formed: read, parse, verify and solve. Each can fail and each will return `Err(String)` in case of failure, otherwise they will transform the `Ok(*)` value until it becomes `Ok(Answer)`. Simple, isn't it?

Now, folder/file structure revisited with this knowledge. E.g. day 10. In `./src/day_10` folder there will be specific implementations defining the elements of the pipeline for day 10, such as the parsing logic and solving logic for both part 1 and 2 (and maybe verifier logic), the data model (usually in `model.rs` file), as well as any additional logic that might be shared by both parts in solving etc. In file `./src/day_10.rs` those pieces will be brought together to form a pipeline, usually exposed through a public method `register` that registers the pipeline to the "pipeline manager" (the component that upon running the code gathers all pipelines and based on command line arguments decides which pipeline to invoke).

## Testing

With this pipelined-approach to days, there's a lot of helper methods in `./src/testing` folder to make it easier to write full-pipeline integration-like tests on day-basis. Each day is usually tested against the example given by the AOC 2024 website.

## More README

I will not pack all the information in this README - too much to share. So navigate throught the folder structure of the project to find more READMEs where appropriate.

They are available at the following folders:
- day specific code folder (such as `./src/day_05`) and they will describe ideas used to solve the problem as well as any other interesting bits, with information on time needed to run the pipelines on the actual input that is specific to me. The time is measured as follows, I run the program against a day input and select part 1 or 2 using `Measure-Command` on Windows - I repeat that few times and take one of the stats rounded up/down or not to my liking. Important: I will usually not describe what the problem is, the problem statements are available on the AOC 2024 website, so I will tend to assume knowledge of what the problem is, and focus on other things.