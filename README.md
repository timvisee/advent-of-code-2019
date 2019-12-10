# Advent of Code 2019 in Rust
My [Advent of Code 2019][aoc-2019] solutions in the Rust programming language.
This repository holds a separate Rust project for each day and part.

This year I attempt to develop a short, compact and fast solution for each
problem.

## Timings
Here is how long each solution takes to run to completion.
All solutions are measured non scientifically with [`hyperfine`][hyperfine], on
a `i5-4670k @ 3.9Ghz` machine running Linux.

|        | Part A                          | Part B                          |
|:-------|:-------------------------------:|:-------------------------------:|
| day 1  | [`0.2ms`](./day01/src/main.rs)  | [`0.2ms`](./day01/src/main.rs)  |
| day 2  | [`0.2ms`](./day02/src/main.rs)  | [`1.1ms`](./day02/src/main.rs)  |
| day 3  | [`0.4ms`](./day03/src/main.rs)  | [`0.5ms`](./day03/src/main.rs)  |
| day 4  | [`22.3ms`](./day04/src/main.rs) | [`22.9ms`](./day04/src/main.rs) |
| day 5  | [`0.2ms`](./day05/src/main.rs)  | [`0.2ms`](./day05/src/main.rs)  |
| day 6  | [`6.9ms`](./day06/src/main.rs)  | [`0.6ms`](./day06/src/main.rs)  |
| day 7  | [`0.7ms`](./day07/src/main.rs)  | [`1.7ms`](./day07/src/main.rs)  |
| day 8  | [`0.1ms`](./day08/src/main.rs)  | [`0.2ms`](./day08/src/main.rs)  |
| day 9  | [`0.2ms`](./day09/src/main.rs)  | [`9.2ms`](./day09/src/main.rs)  |
| day 10 | [`2.4ms`](./day10/src/main.rs)  | [`2.4ms`](./day10/src/main.rs)  |

## Run solutions
Each Rust project contains a `input.txt` file, holding the puzzle input. Simply
run the project to see the solution appear.

```bash
# Switch to day 4a, and run it
cd day04a
cargo run --release
```

Some solutions might require Rust Nightly.

## License
This project is released under the GNU GPL-3.0 license.
Check out the [LICENSE](LICENSE) file for more information.

[aoc-2019]: https://adventofcode.com/2019
[hyperfine]: https://github.com/sharkdp/hyperfine
