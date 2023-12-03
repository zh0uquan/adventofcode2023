use std::collections::{HashMap, HashSet};
use std::fmt::{Display, Formatter};

fn main() {
    let input = include_str!("input.txt");
    println!("{:?}", gear_sum(input));
}

#[derive(Debug, PartialEq, Hash, Eq, Clone)]
struct Number {
    number: u32,
    row: usize,
    start: usize,
    end: usize,
}

#[derive(Debug)]
struct Grid {
    grid: Vec<Vec<char>>
}

impl Display for Grid {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for row in self.grid.iter() {
            writeln!(f, "{}", String::from_iter(row))?;
        }
        Ok(())
    }
}

fn gear_sum(input: &str) -> (u32, u32) {
    let matrix: Vec<Vec<char>> =
        input.lines().map(|line| line.chars().collect()).collect();
    let mut bucket: HashMap<usize, Vec<Number>> = HashMap::new();
    // m +- 1
    for (row_number, row) in matrix.iter().enumerate() {
        let mut temp = String::from("");
        let mut start = 0;
        for (index, ch) in row.iter().enumerate() {
            if ch.is_ascii_digit() {
                if temp.is_empty() {
                    start = index;
                }
                temp.push(*ch);
            } else if !temp.is_empty() {
                let number = Number {
                    start,
                    row: row_number,
                    end: index - 1,
                    number: temp.parse::<u32>().unwrap(),
                };
                bucket.entry(row_number).or_default().push(number);
                temp = String::from("");
            }
        }
        if !temp.is_empty() {
            let number = Number {
                start,
                row: row_number,
                end: row.len() - 1,
                number: temp.parse::<u32>().unwrap(),
            };
            bucket.entry(row_number).or_default().push(number);
        }
    }

    let mut chars: HashMap<(usize, usize), Vec<u32>> = HashMap::new();
    for (row_number, row) in matrix.iter().enumerate() {
        for (index, ch) in row.iter().enumerate() {
            match ch {
                '0'..='9' => continue,
                '.' => continue,
                _ => {
                    for diff in [-1, 0, 1] {
                        if let Some(row) = row_number.checked_add_signed(diff) {
                            bucket
                                .entry(row)
                                .or_default()
                                .iter()
                                .filter(|n| {
                                    let start = n.start as i32;
                                    let end = n.end as i32;
                                    start.max(index as i32 - 1) <= end.min(index as i32 + 1)
                                })
                                .for_each(|n| {
                                    chars.entry((row_number, index)).or_default().push(n.number);
                                })
                        }
                    }
                }
            }
        }
    }

    let part1 = chars
        .values()
        .map(|v| v.iter().sum::<u32>())
        .sum();

    let part2 = chars
        .values()
        .filter(|v| v.len() == 2)
        .map(|v| v.iter().product::<u32>())
        .sum();

    (part1, part2)
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_main() {
        let input = indoc! {
            r#"
            467..114..
            ...*......
            ..35..633.
            ......#...
            617*......
            .....+.58.
            ..592.....
            ......755.
            ...$.*....
            .664.598..
            "#
        };
        assert_eq!((4361, 467835), gear_sum(input))
    }
}
