use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{space0, space1, u32 as nom_u32};
use nom::error::Error;
use nom::multi::separated_list1;
use nom::sequence::{preceded, separated_pair};
use nom::IResult;

fn main() {
    let input = include_str!("input.txt");
    println!("{}", part1(input));
    println!("{}", part2(input));
}

#[derive(Default)]
struct Game {
    id: u32,
    blue: u32,
    red: u32,
    green: u32,
}

type ColorPairs<'a> = Vec<(u32, &'a str)>;

fn parse_game(
    input: &str,
) -> IResult<&str, (u32, Vec<ColorPairs<'_>>), Error<&str>> {
    separated_pair(preceded(tag("Game "), nom_u32), tag(":"), parse_colors)(
        input,
    )
}

fn parse_colors(
    input: &str,
) -> IResult<&str, Vec<ColorPairs<'_>>, Error<&str>> {
    separated_list1(tag(";"), parse_color_pairs)(input)
}

fn parse_color_pairs(input: &str) -> IResult<&str, ColorPairs, Error<&str>> {
    separated_list1(tag(","), preceded(space1, parse_color_number))(input)
}

fn parse_color_number(input: &str) -> IResult<&str, (u32, &str), Error<&str>> {
    separated_pair(
        nom_u32,
        tag(" "),
        alt((tag("blue"), tag("red"), tag("green"))),
    )(input)
}

impl Game {
    fn new(line: &str) -> Self {
        let (_, (id, pairs)) = parse_game(line).unwrap();
        let (mut red, mut green, mut blue) = (0, 0, 0);
        for colors in pairs {
            for (number, color) in colors {
                match color {
                    "red" => red = red.max(number),
                    "blue" => blue = blue.max(number),
                    "green" => green = green.max(number),
                    _ => unreachable!(),
                }
            }
        }
        Game {
            id,
            red,
            green,
            blue,
        }
    }

    fn power_of_game(&self) -> u32 {
        self.red * self.green * self.blue
    }
}

fn part1(input: &str) -> u32 {
    input
        .lines()
        .map(Game::new)
        .filter(|g| g.red <= 12 && g.green <= 13 && g.blue <= 14)
        .map(|g| g.id)
        .sum()
}

fn part2(input: &str) -> u32 {
    input
        .lines()
        .map(Game::new)
        .map(|g| g.power_of_game())
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
        Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
        "#
        }
    }

    #[rstest]
    fn test_part1(input: &str) {
        assert_eq!(8, part1(input))
    }

    #[rstest]
    fn test_part2(input: &str) {
        assert_eq!(2286, part2(input))
    }
}
