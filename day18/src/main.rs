use itertools::Itertools;

fn main() {
    let input = include_str!("input.txt");
    println!("{:?}", part1(input));
    println!("{:?}", part2(input));
}

pub trait ShoelaceArea {
    fn shoelace(&self) -> isize;
}

impl<T> ShoelaceArea for T
where
    T: AsRef<[(isize, isize)]>,
{
    fn shoelace(&self) -> isize {
        let pts = self.as_ref();
        pts.iter()
            .tuple_windows()
            .fold(0, |mut acc, (p1, p2)| {
                acc += p1.0 * p2.1 - p1.1 * p2.0;
                acc
            })
            .abs()
            / 2
    }
}

fn part1(input: &str) -> isize {
    let mut perimeter = 0;
    let positions: Vec<(isize, isize)> =
        input.lines().fold(vec![(0, 0)], |mut acc, line| {
            let (direction, distance, _color) =
                line.split_whitespace().collect_tuple().unwrap();
            let distance = distance.parse::<isize>().unwrap();
            let (last_i, last_j) = *acc.last().unwrap();
            let (i, j) = match direction {
                "R" => (last_i, last_j + distance),
                "L" => (last_i, last_j - distance),
                "D" => (last_i + distance, last_j),
                "U" => (last_i - distance, last_j),
                _ => panic!("disco!"),
            };
            acc.push((i, j));
            perimeter += distance;
            acc
        });
    positions.shoelace() + perimeter / 2 + 1
}

fn part2(input: &str) -> isize {
    let mut perimeter = 0;
    let positions: Vec<(isize, isize)> =
        input.lines().fold(vec![(0, 0)], |mut acc, line| {
            let (_direction, _distance, color) =
                line.split_whitespace().collect_tuple().unwrap();
            let color = color.trim_matches(|c| c == '(' || c == ')');
            let (distance, direction) = color.split_at(color.len() - 1);
            let distance =
                isize::from_str_radix(distance.trim_start_matches('#'), 16)
                    .unwrap();
            let (last_i, last_j) = *acc.last().unwrap();
            let (i, j) = match direction {
                "0" => (last_i, last_j + distance),
                "1" => (last_i + distance, last_j),
                "2" => (last_i, last_j - distance),
                "3" => (last_i - distance, last_j),
                _ => panic!("disco!"),
            };
            acc.push((i, j));
            perimeter += distance;
            acc
        });
    positions.shoelace() + perimeter / 2 + 1
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_solve() {
        let input = indoc! {
            r#"
            R 6 (#70c710)
            D 5 (#0dc571)
            L 2 (#5713f0)
            D 2 (#d2c081)
            R 2 (#59c680)
            D 2 (#411b91)
            L 5 (#8ceee2)
            U 2 (#caa173)
            L 1 (#1b58a2)
            U 2 (#caa171)
            R 2 (#7807d2)
            U 3 (#a77fa3)
            L 2 (#015232)
            U 2 (#7a21e3)
            "#
        };
        assert_eq!(part1(input), 62);
        assert_eq!(part2(input), 952408144115);
    }
}
