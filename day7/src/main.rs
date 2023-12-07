use std::cmp::Ordering;
use std::collections::HashMap;
use std::iter::zip;
use itertools::Itertools;

fn main() {
    let input = include_str!("input.txt");
    println!("{}", part1(input));
    println!("{}", part2(input));
}

fn part1(input: &str) -> u32 {
    let solution = Solution {
        part: Part::Part1
    };
    get_total_winnings(input, solution)
}

fn part2(input: &str) -> u32 {
    let solution = Solution {
        part: Part::Part2
    };
    get_total_winnings(input, solution)
}



fn get_total_winnings(input: &str, solution: Solution) -> u32 {
    input.lines()
        .map(|line| {
            let (label, bid_str) = line.split_once(' ').unwrap();
            solution.to_card(label, bid_str)
        })
        .sorted_by(|a, b| solution.compare(a, b))
        .enumerate()
        .map(|(rank, card)| {
            // println!("{:?} {:?}", rank + 1, card);
            (rank + 1) as u32 * card.bid
        }
        )
        .sum()
}


#[derive(Debug)]
struct Card<'a> {
    label: &'a str,
    bid: u32,
    card_type: CardType,
}

#[derive(PartialEq, PartialOrd, Debug)]
enum CardType {
    FiveKind = 7,
    FourKind = 6,
    FullHouse = 5,
    ThreeKind = 4,
    TwoPair = 3,
    OnePair = 2,
    HighCard = 1,
}

enum Part {
    Part1,
    Part2,
}

struct Solution {
    part: Part,
}

impl Solution {
    fn compare(&self, a: &Card, b: &Card) -> Ordering {
        let convert_char = |c: char| -> u32 {
            if c.is_ascii_digit() {
                return c.to_digit(10).unwrap();
            }
            match c {
                'A' => 14,
                'K' => 13,
                'Q' => 12,
                'J' => {
                    match self.part {
                        Part::Part1 => 11,
                        Part::Part2 => 1
                    }
                },
                'T' => 10,
                _ => unreachable!()
            }
        };
        let cmp_label = |a: &str, b: &str| -> Ordering {
            for (c1, c2) in zip(a.chars(), b.chars()) {
                let (d1, d2) = (convert_char(c1), convert_char(c2));
                match d1.cmp(&d2) {
                    Ordering::Equal => continue,
                    order => return order,
                }
            }
            Ordering::Equal
        };

        match a.card_type.partial_cmp(&b.card_type) {
            Some(order) => match order {
                Ordering::Equal => cmp_label(a.label, b.label),
                _ => order,
            }
            None => unreachable!()
        }
    }

    fn to_card<'a>(&self, label: &'a str, bid_str: &'a str) -> Card<'a> {
        Card {
            label,
            bid: bid_str.parse::<u32>().expect("should be a number"),
            card_type: self.to_card_type(label)
        }
    }

    fn to_card_type(&self, label: &str) -> CardType {
        let mut counters: HashMap<char, usize> = label.chars().counts();
        let count = match self.part {
            Part::Part1 => {
                let mut count: Vec<usize> = counters.into_values().collect();
                count.sort();
                count
            },
            Part::Part2 => {
                let n_j = counters.remove(&'J').unwrap_or(0);
                let mut count: Vec<usize> = counters.into_values().collect();
                count.sort();
                if count.is_empty() {
                    count = vec![5];
                } else {
                    *count.last_mut().unwrap() += n_j;
                }
                count
            }
        };

        match count[..] {
            [5] => CardType::FiveKind,
            [1, 4] => CardType::FourKind,
            [2, 3] => CardType::FullHouse,
            [1, 1, 3] => CardType::ThreeKind,
            [1, 2, 2] => CardType::TwoPair,
            [1, 1, 1, 2] => CardType::OnePair,
            [1, 1, 1, 1, 1] => CardType::HighCard,
            _ => unreachable!()
        }
    }

}

#[cfg(test)]
mod tests {
    use indoc::indoc;
    use rstest::{fixture, rstest};
    use super::*;

    #[fixture]
    fn solution_part1() -> Solution {
        Solution {
            part: Part::Part1
        }
    }

    #[fixture]
    fn solution_part2() -> Solution {
        Solution {
            part: Part::Part2
        }
    }

    #[test]
    fn test_card_type() {
        assert!(CardType::HighCard < CardType::FiveKind);
        assert!(CardType::HighCard < CardType::FullHouse);
        assert_eq!(CardType::HighCard, CardType::HighCard);
    }

    #[rstest]
    fn test_part1() {
        let input = indoc! {
            r#"
            32T3K 765
            T55J5 684
            KK677 28
            KTJJT 220
            QQQJA 483
            "#
        };
        assert_eq!(6440, part1(input));
    }

    #[rstest]
    fn test_part2() {
        let input = indoc! {
            r#"
            32T3K 765
            T55J5 684
            KK677 28
            KTJJT 220
            QQQJA 483
            "#
        };
        assert_eq!(5905, part2(input));
    }

    #[rstest]
    fn test_order(solution_part1: Solution, solution_part2: Solution) {
        assert_eq!(
            solution_part1.compare(
                &solution_part1.to_card("AAAA2", "12"),
                &solution_part1.to_card("2222A", "1"),
            ),
            Ordering::Greater
        );
        assert_eq!(
            solution_part2.compare(
                &solution_part2.to_card("JJJJJ", "1"),
                &solution_part2.to_card("2222A", "1"),
            ),
            Ordering::Greater
        );
        assert_eq!(
            solution_part2.compare(
                &solution_part2.to_card("JJJJJ", "1"),
                &solution_part2.to_card("2JJJJ", "1"),
            ),
            Ordering::Less
        )
    }

    #[rstest]
    fn test_parse_card_type(solution_part1: Solution) {
        assert_eq!(solution_part1.to_card_type("AAAAA"), CardType::FiveKind);
        assert_eq!(solution_part1.to_card_type("AA8AA"), CardType::FourKind);
        assert_eq!(solution_part1.to_card_type("23332"), CardType::FullHouse);
        assert_eq!(solution_part1.to_card_type("TTT98"), CardType::ThreeKind);
        assert_eq!(solution_part1.to_card_type("23432"), CardType::TwoPair);
        assert_eq!(solution_part1.to_card_type("A23A4"), CardType::OnePair);
        assert_eq!(solution_part1.to_card_type("23456"), CardType::HighCard);
    }
}