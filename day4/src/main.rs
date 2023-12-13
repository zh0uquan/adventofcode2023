use nom::bytes::complete::tag;
use nom::character::complete::{space0, space1, u32 as nom_u32};
use nom::error::Error;
use nom::multi::separated_list1;
use nom::sequence::{preceded, separated_pair};
use nom::IResult;

fn main() {
    let input = include_str!("input.txt");
    println!("{:?}", part1(input));
    println!("{:?}", part2(input));
}

#[derive(Debug)]
struct Card {
    id: u32,
    winning_numbers: Vec<u32>,
    owning_numbers: Vec<u32>,
}

fn parse_card(input: &str) -> IResult<&str, Card> {
    let (input, (id, (winning_numbers, owning_numbers))) = separated_pair(
        parse_card_id,
        tag(": "),
        separated_pair(parse_numbers, tag(" | "), parse_numbers),
    )(input)?;
    Ok((
        input,
        Card {
            id,
            winning_numbers,
            owning_numbers,
        },
    ))
}

fn parse_card_id(input: &str) -> IResult<&str, u32, Error<&str>> {
    preceded(tag("Card"), preceded(space1, nom_u32))(input)
}

fn parse_numbers(input: &str) -> IResult<&str, Vec<u32>, Error<&str>> {
    separated_list1(space1, preceded(space0, nom_u32))(input)
}

fn part1(input: &str) -> u32 {
    input
        .lines()
        .map(|line| parse_card(line).unwrap().1)
        .map(|card| {
            card.owning_numbers.iter().fold(0u32, |mut acc, n| {
                if card.winning_numbers.contains(n) {
                    acc = if acc == 0 { 1 } else { acc * 2 }
                }
                acc
            })
        })
        .sum()
}

fn part2(input: &str) -> u32 {
    let cards: Vec<Card> = input
        .lines()
        .map(|line| parse_card(line).unwrap().1)
        .collect();
    cards
        .iter()
        .map(|card| {
            card.owning_numbers.iter().fold(0u32, |mut acc, n| {
                if card.winning_numbers.contains(n) {
                    acc = if acc == 0 { 1 } else { acc + 1 }
                }
                acc
            }) as usize
        })
        .enumerate()
        .fold(vec![1; cards.len()], |mut acc, (i, copy_nums)| {
            (i + 1..=i + copy_nums)
                .filter(|n| *n < cards.len())
                .for_each(|n| acc[n] += acc[i]);
            acc
        })
        .iter()
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;
    use rstest::*;

    #[fixture]
    pub fn input() -> &'static str {
        indoc! {
            r#"
        Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
        Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
        Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
        Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
        Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
        Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
        "#
        }
    }

    #[rstest]
    fn test_part1(input: &str) {
        assert_eq!(13, part1(input))
    }

    #[rstest]
    fn test_part2(input: &str) {
        assert_eq!(30, part2(input))
    }
}
