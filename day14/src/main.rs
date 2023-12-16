use std::fmt::Display;

fn main() {
    let input = include_str!("input.txt");
    println!("{:?}", day14::part1(input));
    println!("{:?}", day14::part2(input));
}
