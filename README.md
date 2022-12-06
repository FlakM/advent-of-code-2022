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

## Running benchmarks


```shell
# remove everything after 2> to have more complete reports
BENCH=true cargo bench 2>/dev/null | grep "time:"
```
