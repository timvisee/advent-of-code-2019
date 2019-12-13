# Advent of Code 2019 in Rust
My [Advent of Code 2019][aoc-2019] solutions in the Rust programming language.
This repository holds a separate Rust project for each day and part.

This year I attempt to develop a short, compact and fast solution for each
problem.

## Timings
Here is how long each solution takes to run to completion.
All solutions are measured (non scientifically) with [`hyperfine`][hyperfine] on
a `i5-4670k @ 3.9Ghz` machine running Linux.
Timings include binary execution, input reading and result printing delays.

|                                                | part A                          | part B                           |
|:-----------------------------------------------|:--------------------------------|:---------------------------------|
| [day 1](https://adventofcode.com/2019/day/1)   | [`0.2ms`](./day01a/src/main.rs) | [`0.2ms`](./day01b/src/main.rs)  |
| [day 2](https://adventofcode.com/2019/day/2)   | [`0.2ms`](./day02a/src/main.rs) | [`1.1ms`](./day02b/src/main.rs)  |
| [day 3](https://adventofcode.com/2019/day/3)   | [`0.4ms`](./day03a/src/main.rs) | [`0.5ms`](./day03b/src/main.rs)  |
| [day 4](https://adventofcode.com/2019/day/4)   | [`0.9ms`](./day04a/src/main.rs) | [`1.1ms`](./day04b/src/main.rs)  |
| [day 5](https://adventofcode.com/2019/day/5)   | [`0.2ms`](./day05a/src/main.rs) | [`0.2ms`](./day05b/src/main.rs)  |
| [day 6](https://adventofcode.com/2019/day/6)   | [`6.9ms`](./day06a/src/main.rs) | [`0.6ms`](./day06b/src/main.rs)  |
| [day 7](https://adventofcode.com/2019/day/7)   | [`0.7ms`](./day07a/src/main.rs) | [`1.7ms`](./day07b/src/main.rs)  |
| [day 8](https://adventofcode.com/2019/day/8)   | [`0.1ms`](./day08a/src/main.rs) | [`0.2ms`](./day08b/src/main.rs)  |
| [day 9](https://adventofcode.com/2019/day/9)   | [`0.2ms`](./day09a/src/main.rs) | [`9.2ms`](./day09b/src/main.rs)  |
| [day 10](https://adventofcode.com/2019/day/10) | [`2.4ms`](./day10a/src/main.rs) | [`2.4ms`](./day10b/src/main.rs)  |
| [day 11](https://adventofcode.com/2019/day/11) | [`5.1ms`](./day11a/src/main.rs) | [`1.1ms`](./day11b/src/main.rs)  |
| [day 12](https://adventofcode.com/2019/day/12) | [`0.6ms`](./day12a/src/main.rs) | [`23.7ms`](./day12b/src/main.rs) |
| [day 13](https://adventofcode.com/2019/day/13) | [`0.9ms`](./day13a/src/main.rs) | [`23.1ms`](./day13b/src/main.rs) |

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
