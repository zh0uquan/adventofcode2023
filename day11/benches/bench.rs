use day11::{process, process_manhattan};
fn main() {
    // Run registered benchmarks.
    divan::main();
}

#[divan::bench]
fn bench_part1() {
    let input = include_str!("../src/input.txt");
    process_manhattan(divan::black_box(input), divan::black_box(2));
}

#[divan::bench]
fn bench_part2() {
    let input = include_str!("../src/input.txt");
    process_manhattan(
        divan::black_box(input),
        divan::black_box(1000000),
    );
}
