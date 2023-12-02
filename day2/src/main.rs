use itertools::Itertools;

fn main() {
    let input = include_str!("input.txt");
    println!("{}", part1(input));
    println!("{}", part2(input));
}

#[derive(Default)]
struct Game {
    id: usize,
    blue: usize,
    red: usize,
    green: usize,
}

impl Game {
    fn new(line: &str) -> Self {
        let (game, cubes) = line.split(':').collect_tuple().unwrap();
        let (_, id) = game.split_whitespace().collect_tuple().unwrap();
        let id = id.parse().unwrap();
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;
        for colors in cubes.split(';') {
            for color in colors.split(',') {
                let (number, color) = color
                    .strip_prefix(' ')
                    .unwrap()
                    .split_whitespace()
                    .collect_tuple()
                    .unwrap();

                let number = number.parse().unwrap();
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

    fn power_of_game(&self) -> usize {
        self.red * self.green * self.blue
    }
}

fn part1(input: &str) -> usize {
    input
        .lines()
        .map(Game::new)
        .filter(|g| g.red <= 12 && g.green <= 13 && g.blue <= 14)
        .map(|g| g.id)
        .sum()
}

fn part2(input: &str) -> usize {
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
