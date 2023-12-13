use day11::{process, process_manhattan};

fn main() {
    let input = include_str!("input.txt");
    // println!("{}", process(input, 2));
    // println!("{}", process(input, 1000000));
    println!("{}", process_manhattan(input, 2));
    println!("{}", process_manhattan(input, 1000000))
}
