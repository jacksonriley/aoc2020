Advent of Code 2020
===================

My solutions for [Advent of Code 2020](https://adventofcode.com/2020).

Planning to do this year in Rust, we'll see if I run out of steam!

Input for each day goes in `input/<day>`.

The binary for each day can be run with `cargo run --bin <day>`.
This will output the solutions for part 1 and part 2.

Unit tests can be run with `cargo test --bin <day>`.

## Timings:


Work Laptop (Lenovo Thinkpad X1 Carbon)
| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `./target/release/01` | 4.6 ± 0.3 | 4.0 | 6.0 | 1.03 ± 0.09 |
| `./target/release/02` | 4.8 ± 0.3 | 4.4 | 6.3 | 1.07 ± 0.08 |
| `./target/release/03` | 4.6 ± 0.3 | 4.1 | 5.8 | 1.02 ± 0.08 |
| `./target/release/04` | 5.0 ± 0.5 | 4.5 | 12.6 | 1.12 ± 0.13 |
| `./target/release/05` | 4.6 ± 0.3 | 4.1 | 6.1 | 1.02 ± 0.08 |
| `./target/release/06` | 4.7 ± 0.3 | 4.2 | 6.5 | 1.04 ± 0.09 |
| `./target/release/07` | 5.9 ± 0.3 | 5.4 | 8.4 | 1.31 ± 0.11 |
| `./target/release/08` | 7.0 ± 0.6 | 6.4 | 11.4 | 1.56 ± 0.16 |
| `./target/release/09` | 4.7 ± 0.3 | 4.3 | 6.8 | 1.06 ± 0.09 |
| `./target/release/10` | 4.5 ± 0.2 | 4.1 | 5.7 | 1.00 |
| `./target/release/11` | 50.8 ± 0.6 | 49.8 | 52.2 | 11.35 ± 0.63 |
| `./target/release/12` | 4.5 ± 0.3 | 4.1 | 6.3 | 1.01 ± 0.09 |
| `./target/release/13` | 4.5 ± 0.3 | 4.0 | 6.4 | 1.00 ± 0.09 |
| `./target/release/14` | 14.2 ± 0.5 | 13.4 | 16.8 | 3.18 ± 0.20 |
| `./target/release/15` | 620.2 ± 7.1 | 610.2 | 643.2 | 138.59 ± 7.65 |
| `./target/release/16` | 7.7 ± 0.4 | 6.9 | 11.5 | 1.72 ± 0.13 |
| `./target/release/17` | 38.7 ± 0.7 | 37.4 | 40.1 | 8.66 ± 0.49 |
| `./target/release/18` | 5.8 ± 0.5 | 5.3 | 12.4 | 1.29 ± 0.13 |
| `./target/release/19` | 18.4 ± 0.8 | 17.2 | 24.8 | 4.11 ± 0.29 |
| `./target/release/20` | 5.8 ± 0.3 | 5.3 | 8.0 | 1.30 ± 0.10 |
| `./target/release/21` | 5.4 ± 0.3 | 4.9 | 6.8 | 1.21 ± 0.09 |
| `./target/release/22` | 132.9 ± 5.0 | 129.8 | 154.0 | 29.70 ± 1.96 |
| `./target/release/23` | 223.2 ± 3.4 | 216.9 | 229.3 | 49.87 ± 2.80 |
| `./target/release/24` | 98.3 ± 3.5 | 95.5 | 111.2 | 21.96 ± 1.42 |
| `./target/release/25` | 5.1 ± 0.3 | 4.6 | 7.0 | 1.14 ± 0.10 |


Personal laptop (2014 Macbook Air)
| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `./target/release/01` | 3.6 ± 0.5 | 2.9 | 6.1 | 1.00 |
| `./target/release/02` | 4.1 ± 0.6 | 3.3 | 7.1 | 1.15 ± 0.24 |
| `./target/release/03` | 3.8 ± 0.8 | 3.0 | 10.0 | 1.05 ± 0.28 |
| `./target/release/04` | 4.3 ± 0.6 | 3.5 | 7.5 | 1.22 ± 0.26 |
| `./target/release/05` | 3.6 ± 0.5 | 2.9 | 5.8 | 1.00 ± 0.20 |
| `./target/release/06` | 3.8 ± 0.6 | 3.0 | 6.8 | 1.06 ± 0.24 |
| `./target/release/07` | 5.6 ± 0.8 | 4.8 | 11.5 | 1.57 ± 0.34 |
| `./target/release/08` | 7.6 ± 0.7 | 6.7 | 11.5 | 2.12 ± 0.38 |
| `./target/release/09` | 3.9 ± 0.6 | 3.2 | 7.2 | 1.10 ± 0.24 |
| `./target/release/10` | 3.6 ± 0.6 | 2.9 | 7.1 | 1.01 ± 0.22 |
| `./target/release/11` | 78.7 ± 3.4 | 76.8 | 97.7 | 22.11 ± 3.52 |
| `./target/release/12` | 3.7 ± 0.5 | 3.0 | 6.4 | 1.03 ± 0.21 |
| `./target/release/13` | 3.7 ± 0.7 | 2.9 | 8.8 | 1.03 ± 0.25 |
| `./target/release/14` | 15.5 ± 1.0 | 14.4 | 19.7 | 4.36 ± 0.73 |
| `./target/release/15` | 1033.7 ± 15.9 | 1013.9 | 1065.4 | 290.33 ± 44.67 |
| `./target/release/16` | 8.6 ± 1.1 | 7.6 | 18.9 | 2.42 ± 0.48 |
| `./target/release/17` | 100.1 ± 2.0 | 97.3 | 105.5 | 28.11 ± 4.34 |
| `./target/release/18` | 6.1 ± 0.6 | 5.4 | 9.1 | 1.72 ± 0.31 |
| `./target/release/19` | 27.2 ± 1.3 | 25.4 | 33.9 | 7.63 ± 1.22 |
| `./target/release/20` | 7.1 ± 0.8 | 6.2 | 12.9 | 2.00 ± 0.38 |
| `./target/release/21` | 5.4 ± 0.5 | 4.8 | 8.1 | 1.53 ± 0.27 |
| `./target/release/22` | 216.3 ± 3.9 | 212.3 | 227.6 | 60.75 ± 9.37 |
| `./target/release/23` | 522.2 ± 27.9 | 494.8 | 608.8 | 146.65 ± 23.78 |
| `./target/release/24` | 125.2 ± 2.5 | 123.3 | 135.3 | 35.17 ± 5.43 |
| `./target/release/25` | 4.0 ± 0.5 | 3.4 | 7.1 | 1.13 ± 0.22 |

Created using
```
hyperfine --warmup 3 -L day 01,02,03,04,05,06,07,08,09,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25 './target/release/{day}' --export-markdown dd --min-runs 20
```