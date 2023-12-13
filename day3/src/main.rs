use std::collections::HashMap;
use std::fmt::{Display, Formatter};

use regex::Regex;

fn main() {
    let input = include_str!("input.txt");
    println!("{:?}", gear_sum(input));
    println!("{:?}", gear_sum_v2(input));
}

#[derive(Debug, Hash, Clone)]
struct Number {
    number: u32,
    row: usize,
    start: usize,
    end: usize,
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
            if ch.is_ascii_digit() || ch == &'.' {
                continue;
            }
            for diff in [-1, 0, 1] {
                if let Some(row) = row_number.checked_add_signed(diff) {
                    bucket
                        .entry(row)
                        .or_default()
                        .iter()
                        .filter(|n| {
                            let start = n.start as i32;
                            let end = n.end as i32;
                            start.max(index as i32 - 1)
                                <= end.min(index as i32 + 1)
                        })
                        .for_each(|n| {
                            chars
                                .entry((row_number, index))
                                .or_default()
                                .push(n.number);
                        })
                }
            }
        }
    }

    let part1 = chars.values().map(|v| v.iter().sum::<u32>()).sum();

    let part2 = chars
        .values()
        .filter(|v| v.len() == 2)
        .map(|v| v.iter().product::<u32>())
        .sum();

    (part1, part2)
}

type CharPos = (usize, usize);

#[derive(Debug)]
struct Grid<'a> {
    grid: Vec<&'a str>,
}

impl<'a> Display for Grid<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for row in self.grid.iter() {
            writeln!(f, "{}", row)?;
        }
        Ok(())
    }
}
impl<'a> Grid<'a> {
    fn get_numbers_bucket(&self) -> HashMap<usize, Vec<Number>> {
        let num_regex = Regex::new(r"\d+").unwrap();
        self.grid
            .iter()
            .enumerate()
            .map(|(r, line)| {
                num_regex
                    .find_iter(line)
                    .map(move |m| {
                        (
                            r,
                            Number {
                                number: m.as_str().parse().unwrap(),
                                start: m.start(),
                                end: m.end() - 1,
                                row: r,
                            },
                        )
                    })
                    .collect::<Vec<(usize, Number)>>()
            })
            .fold(HashMap::new(), |mut acc, v| {
                v.iter().for_each(|(r, n)| {
                    acc.entry(*r).or_default().push(n.clone())
                });
                acc
            })
    }

    fn get_adjacent(
        &self,
        bucket: &HashMap<usize, Vec<Number>>,
    ) -> HashMap<CharPos, Vec<u32>> {
        let mut chars: HashMap<CharPos, Vec<u32>> = HashMap::new();
        self.grid.iter().enumerate().for_each(|(r, line)| {
            line.chars()
                .enumerate()
                .filter(|(_, ch)| !ch.is_ascii_digit() && ch != &'.')
                .for_each(|(c, _)| {
                    for diff in [-1, 0, 1] {
                        if let Some(row) = r.checked_add_signed(diff) {
                            bucket
                                .get(&row)
                                .unwrap_or(&vec![])
                                .iter()
                                .filter(|n| {
                                    let c = c as i32;
                                    (c - 1).max(n.start as i32)
                                        <= (c + 1).min(n.end as i32)
                                })
                                .for_each(|n| {
                                    chars
                                        .entry((r, c))
                                        .or_default()
                                        .push(n.number);
                                });
                        }
                    }
                })
        });
        chars
    }
}

fn gear_sum_v2(input: &str) -> (u32, u32) {
    let grid = Grid {
        grid: input.lines().collect(),
    };
    let bucket = grid.get_numbers_bucket();
    let adjacent = grid.get_adjacent(&bucket);

    (
        adjacent.values().map(|v| v.iter().sum::<u32>()).sum(),
        adjacent
            .values()
            .filter(|v| v.len() == 2)
            .map(|v| v.iter().product::<u32>())
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    #[test]
    fn test_gear_sum() {
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

    #[test]
    fn test_gear_sum_v2() {
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
        assert_eq!((4361, 467835), gear_sum_v2(input))
    }
}
