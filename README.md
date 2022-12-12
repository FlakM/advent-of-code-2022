#  🎅 🎄 advent-of-code-2022 🎅 🎄 

My advent of code solutions https://adventofcode.com/2022

My previous attempt while learning vim finished at day 17 is [here](https://github.com/FlakM/aoc2021)

## Project setup

1. Install rust using
2. Install gihooks: `git config --local core.hooksPath .githooks`
3. Run single task using `./run.sh day1a`


## Adding new day

Following script will automate creating new projects for a brand new
day:

```shell
./new_day.sh 7
```


## Performance:

```
AOC/day1a               time:   [50.207 µs 50.324 µs 50.463 µs]
AOC/day1b               time:   [54.608 µs 54.756 µs 54.926 µs]
AOC/day2a               time:   [56.544 µs 56.690 µs 56.854 µs]
AOC/day2b               time:   [66.157 µs 66.316 µs 66.501 µs]
AOC/day3a               time:   [25.421 µs 25.498 µs 25.581 µs]
AOC/day3b               time:   [32.153 µs 32.277 µs 32.418 µs]
AOC/day4a               time:   [84.456 µs 84.639 µs 84.803 µs]
AOC/day4b               time:   [84.794 µs 84.895 µs 85.016 µs]
AOC/day5a               time:   [192.65 µs 193.10 µs 193.49 µs]
AOC/day5b               time:   [194.54 µs 195.50 µs 196.66 µs]
AOC/day6a               time:   [884.23 ns 885.83 ns 887.61 ns]
AOC/day6b               time:   [6.7763 µs 6.7882 µs 6.8010 µs]
AOC/day7a               time:   [271.41 µs 271.83 µs 272.30 µs]
AOC/day7b               time:   [293.18 µs 293.84 µs 294.82 µs]
AOC/day8a               time:   [1.5971 ms 1.6018 ms 1.6077 ms]
AOC/day8b               time:   [2.0419 ms 2.0472 ms 2.0535 ms]
AOC/day9a               time:   [499.60 µs 500.09 µs 500.64 µs]
AOC/day9b               time:   [624.86 µs 625.28 µs 625.78 µs]
AOC/day10a              time:   [4.0828 µs 4.0881 µs 4.0939 µs]
AOC/day10b              time:   [4.5674 µs 4.5788 µs 4.5940 µs]
AOC/day11a              time:   [2.3165 ms 2.3555 ms 2.4048 ms]
AOC/day11b              time:   [145.41 ms 150.78 ms 156.88 ms]
AOC/day12a              time:   [1.7148 ms 1.7167 ms 1.7188 ms]
AOC/day12b              time:   [87.116 ms 87.218 ms 87.344 ms]
```

## Running benchmarks


```shell
# remove everything after 2> to have more complete reports
BENCH=true cargo bench 2>/dev/null | grep "time:"

# run benches for single day only
BENCH=true cargo bench day6 2>/dev/null | grep "time:"
```
