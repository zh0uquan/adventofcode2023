use std::collections::HashMap;
use std::iter::zip;

fn main() {
    let input = include_str!("input.txt");
    println!("{}", part1(input));
    println!("{}", part2(input));
}

fn part1(input: &str) -> u32 {
    input
        .lines()
        .map(|l| l.chars().filter_map(|c| c.to_digit(10)))
        .map(|mut it| {
            let first = it.next().unwrap();
            let last = it.last().unwrap_or(first);
            first * 10 + last
        })
        .sum()
}

fn part2(input: &str) -> u32 {
    let words = vec![
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    let digits = vec!["1", "2", "3", "4", "5", "6", "7", "8", "9"];
    let mappings = zip(words.clone(), digits.clone()).collect::<HashMap<&str, &str>>();
    let patterns = [&words[..], &digits[..]].concat();

    input
        .lines()
        .map(|line| {
            let mut v: Vec<u32> = vec![];
            let mut index = 0;
            while index < line.len() {
                for p in patterns.iter() {
                    if line[index..].starts_with(p) {
                        let digit = mappings.get(p).unwrap_or(p).parse::<u32>().unwrap();
                        v.push(digit);
                    }
                }
                index += 1;
            }
            v
        })
        .map(|v| {
            [
                v.first().unwrap().to_string(),
                v.last().unwrap().to_string(),
            ]
                .join("")
                .parse::<u32>()
                .unwrap()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_part1() {
        let input = indoc! {
            r#"
            1abc2
            pqr3stu8vwx
            a1b2c3d4e5f
            treb7uchet
            "#
        };
        assert_eq!(142, part1(input))
    }

    #[test]
    fn test_part2() {
        let input = indoc! {
            r#"
            two1nine
            eightwothree
            abcone2threexyz
            xtwone3four
            4nineeightseven2
            zoneight234
            7pqrstsixteen
            "#
        };
        assert_eq!(281, part2(input))
    }
}
