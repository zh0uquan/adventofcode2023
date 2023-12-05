use day5::{part1, part2};

fn main() {
    // Run registered benchmarks.
    divan::main();
}

// Define a `fibonacci` function and register it for benchmarking.
#[divan::bench]
fn bench_part1() {
    let input = include_str!("../src/input.txt");
    part1(divan::black_box(input));
}

#[divan::bench]
fn bench_part2() {
    let input = include_str!("../src/input.txt");
    part2(divan::black_box(input));
}