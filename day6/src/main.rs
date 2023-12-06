use std::iter::zip;
use indoc::indoc;
use roots::Roots;
use roots::find_roots_quadratic;

fn main() {
    let input = indoc! {
        r#"
        Time:        35     69     68     87
        Distance:   213   1168   1086   1248"#
    };
    println!("{:?}", part1_and_part2(input))

}

fn part1_and_part2(input: &str) -> (usize, usize) {
    let time: Vec<&str> = input.lines().next()
        .expect("should have first line")
        .strip_prefix("Time:")
        .expect("should have time prefix")
        .split_whitespace()
        .collect();
    let distance: Vec<&str>= input.lines().nth(1)
        .expect("should have first line")
        .strip_prefix("Distance:")
        .expect("should have distance prefix")
        .split_whitespace()
        .collect();

    let part1_res: usize = zip(time.clone(), distance.clone())
        .map(|(s1, s2)| (s1.parse::<f64>().unwrap(), s2.parse::<f64>().unwrap()))
        .map(|(b, c)| compute_possible_ways(1f64, -b, c))
        .product();


    let part2_res: usize = compute_possible_ways(
        1f64,
        -1.0 * time.join("").parse::<f64>().unwrap(),
            distance.join("").parse::<f64>().unwrap()
    );

    (part1_res, part2_res)

}

fn compute_possible_ways(a: f64, b: f64, c: f64) -> usize {
    let roots = find_roots_quadratic(a, b, c);
    match roots {
        Roots::Two(bound) => {
            let (mut low, mut high) = (*bound.first().unwrap(), *bound.last().unwrap());
            if low.ceil() == low {
                low = low.ceil() + 1.0;
            }
            if high.floor() == high {
                high = high.floor() - 1.0;
            }
            high.floor() as usize + 1 - low.ceil() as usize
        }
        _ => unreachable!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_ways() {
        assert_eq!(compute_possible_ways(1f64, -7f64, 9f64), 4);
        assert_eq!(compute_possible_ways(1f64, -15f64, 40f64), 8);
        assert_eq!(compute_possible_ways(1f64, -30f64, 200f64), 9);
    }

    #[test]
    fn test_part1() {
        let input = indoc! {
            r#"
            Time:      7  15   30
            Distance:  9  40  200
           "#
        };
        assert_eq!(part1_and_part2(input), (288, 71503))
    }
}

