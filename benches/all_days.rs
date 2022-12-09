use criterion::{criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("AOC");
    DAYS.iter().for_each(|(name, fun)| {
        group.bench_function(name.to_string(), |b| b.iter(fun));
    });
    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

static DAYS: &[(&str, fn())] = &[
    ("day1a", day1a::main),
    ("day1b", day1b::main),
    ("day2a", day2a::main),
    ("day2b", day2b::main),
    ("day3a", day3a::main),
    ("day3b", day3b::main),
    ("day4a", day4a::main),
    ("day4b", day4b::main),
    ("day5a", day5a::main),
    ("day5b", day5b::main),
    ("day6a", day6a::main),
    ("day6b", day6b::main),
    ("day7a", day7a::main),
    ("day7b", day7b::main),
    ("day8a", day8a::main),
    ("day8b", day8b::main),
    ("day9a", day9a::main),
    ("day9b", day9b::main),
    //("day10a", day10a::main),
    //("day10b", day10b::main),
    //("day11a", day11a::main),
    //("day11b", day11b::main),
    //("day12a", day12a::main),
    //("day12b", day12b::main),
    //("day13a", day13a::main),
    //("day13b", day13b::main),
    //("day14a", day14a::main),
    //("day14b", day14b::main),
    //("day15a", day15a::main),
    //("day15b", day15b::main),
    //("day16a", day16a::main),
    //("day16b", day16b::main),
    //("day17a", day17a::main),
    //("day17b", day17b::main),
    //("day18a", day18a::main),
    //("day18b", day18b::main),
    //("day19a", day19a::main),
    //("day19b", day19b::main),
    //("day20a", day20a::main),
    //("day20b", day20b::main),
    //("day21a", day21a::main),
    //("day21b", day21b::main),
    //("day22a", day22a::main),
    //("day22b", day22b::main),
    //("day23a", day23a::main),
    //("day23b", day23b::main),
    //("day24a", day24a::main),
    //("day24b", day24b::main),
    //("day25a", day25a::main),
    //("day25b", day25b::main),
];
