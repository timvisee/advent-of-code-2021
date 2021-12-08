# Advent of Code 2021 in Rust

My [Advent of Code 2021][aoc-2021] solutions in the Rust programming language.
This repository holds a separate Rust project for each day and part.

I attempt to develop a standalone, elegant, compact and fast solution for each
problem (two each day).

Previous year I did the same, solving everything in under a second:

- https://timvisee.com/blog/solving-aoc-2020-in-under-a-second/
- https://github.com/timvisee/advent-of-code-2020

## Timings

Here is how long each solution runs. All solutions are measured (non
scientifically) in [`bench.rs`](./runner/src/bin/bench.rs) on an
`AMD Ryzen 9 5900X (24) @ 3.7GHz` machine running Linux.

|                                                | part A                              | part B                              |
|:-----------------------------------------------|:------------------------------------|:------------------------------------|
| [day 1](https://adventofcode.com/2021/day/1)   | [`  0.025ms`](./day01a/src/main.rs) | [`  0.024ms`](./day01b/src/main.rs) |
| [day 2](https://adventofcode.com/2021/day/2)   | [`  0.024ms`](./day02a/src/main.rs) | [`  0.025ms`](./day02b/src/main.rs) |
| [day 3](https://adventofcode.com/2021/day/3)   | [`  0.021ms`](./day03a/src/main.rs) | [`  0.023ms`](./day03b/src/main.rs) |
| [day 4](https://adventofcode.com/2021/day/4)   | [`  0.088ms`](./day04a/src/main.rs) | [`  0.137ms`](./day04b/src/main.rs) |
| [day 5](https://adventofcode.com/2021/day/5)   | [`  0.110ms`](./day05a/src/main.rs) | [`  0.220ms`](./day05b/src/main.rs) |
| [day 6](https://adventofcode.com/2021/day/6)   | [` 0.0027ms`](./day06a/src/main.rs) | [` 0.0028ms`](./day06b/src/main.rs) |
| [day 7](https://adventofcode.com/2021/day/7)   | [`  0.013ms`](./day07a/src/main.rs) | [`  0.013ms`](./day07b/src/main.rs) |
| [day 8](https://adventofcode.com/2021/day/8)   | [`  0.008ms`](./day08a/src/main.rs) |                                     |

## Run solutions

Each Rust project contains a `input.txt` file, holding the puzzle input. Simply
run the project to see the solution appear.

```bash
# Switch to day 1a, and run it
cd day01a
cargo +nightly run --release

# or run everything in parallel
cd ../runner
cargo +nightly run --release --bin runner-par

# or benchmark every day
cd ../runner
cargo +nightly run --release --bin bench
```

Some solutions require Rust Nightly, that's why `+nightly` is included.

## Other years

- [2021](https://github.com/timvisee/advent-of-code-2021) _(current)_
- [2020](https://github.com/timvisee/advent-of-code-2020)
- [2019](https://github.com/timvisee/advent-of-code-2019)
- [2018](https://github.com/timvisee/advent-of-code-2018)
- [2017](https://github.com/timvisee/advent-of-code-2017)

## License

This project is released under the GNU GPL-3.0 license.
Check out the [LICENSE](LICENSE) file for more information.

[aoc-2021]: https://adventofcode.com/2021
