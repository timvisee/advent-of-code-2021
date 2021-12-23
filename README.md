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
| [day 4](https://adventofcode.com/2021/day/4)   | [`  0.075ms`](./day04a/src/main.rs) | [`  0.106ms`](./day04b/src/main.rs) |
| [day 5](https://adventofcode.com/2021/day/5)   | [`  0.110ms`](./day05a/src/main.rs) | [`  0.220ms`](./day05b/src/main.rs) |
| [day 6](https://adventofcode.com/2021/day/6)   | [` 0.0027ms`](./day06a/src/main.rs) | [` 0.0028ms`](./day06b/src/main.rs) |
| [day 7](https://adventofcode.com/2021/day/7)   | [`  0.013ms`](./day07a/src/main.rs) | [`  0.013ms`](./day07b/src/main.rs) |
| [day 8](https://adventofcode.com/2021/day/8)   | [`  0.008ms`](./day08a/src/main.rs) | [`  0.026ms`](./day08b/src/main.rs) |
| [day 9](https://adventofcode.com/2021/day/9)   | [`  0.012ms`](./day09a/src/main.rs) | [`  0.036ms`](./day09b/src/main.rs) |
| [day 10](https://adventofcode.com/2021/day/10) | [`  0.011ms`](./day10a/src/main.rs) | [`  0.015ms`](./day10b/src/main.rs) |
| [day 11](https://adventofcode.com/2021/day/11) | [`  0.019ms`](./day11a/src/main.rs) | [`  0.039ms`](./day11b/src/main.rs) |
| [day 12](https://adventofcode.com/2021/day/12) | [`  0.015ms`](./day12a/src/main.rs) | [`  0.272ms`](./day12b/src/main.rs) |
| [day 13](https://adventofcode.com/2021/day/13) | [`  0.038ms`](./day13a/src/main.rs) | [`  0.044ms`](./day13b/src/main.rs) |
| [day 14](https://adventofcode.com/2021/day/14) | [`  0.007ms`](./day14a/src/main.rs) | [`  0.008ms`](./day14b/src/main.rs) |
| [day 15](https://adventofcode.com/2021/day/15) | [`  1.05 ms`](./day15a/src/main.rs) | [` 37.7  ms`](./day15b/src/main.rs) |
| [day 16](https://adventofcode.com/2021/day/16) | [`  0.002ms`](./day16a/src/main.rs) | [`  0.007ms`](./day16b/src/main.rs) |
| [day 17](https://adventofcode.com/2021/day/17) | [` 0.0002ms`](./day17a/src/main.rs) | [`  0.095ms`](./day17b/src/main.rs) |
| [day 18](https://adventofcode.com/2021/day/18) | [`  0.141ms`](./day18a/src/main.rs) | [`  2.61 ms`](./day18b/src/main.rs) |
| [day 19](https://adventofcode.com/2021/day/19) | [`  1.03 ms`](./day19a/src/main.rs) | [`  1.03 ms`](./day19b/src/main.rs) |
| [day 20](https://adventofcode.com/2021/day/20) | [`  0.042ms`](./day20a/src/main.rs) | [`  3.10 ms`](./day20b/src/main.rs) |
| [day 21](https://adventofcode.com/2021/day/21) | [` 0.0008ms`](./day21a/src/main.rs) | [`  0.016ms`](./day21b/src/main.rs) |
| [day 22](https://adventofcode.com/2021/day/22) | [`  0.083ms`](./day22a/src/main.rs) | [`  1.74 ms`](./day22b/src/main.rs) |

|              | one-by-one (1 CPU core)                  | parallel                                     |
|:-------------|:-----------------------------------------|:---------------------------------------------|
| _everything_ | [`50.36 ms`](./runner/src/bin/runner.rs) | [`39.53ms`](./runner/src/bin/runner-par.rs)  |

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
