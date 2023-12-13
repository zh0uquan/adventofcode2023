use itertools::Itertools;

fn main() {
    let input = include_str!("input.txt");
    println!("{:?}", process(input));
}

fn process(input: &str) -> (i32, i32) {
    let mut sum = 0;
    let mut sum_2 = 0;
    for history in input.lines().map(|line| {
        line.split_whitespace()
            .map(|s| s.parse::<i32>().unwrap())
            .collect::<Vec<i32>>()
    }) {
        let mut vec: Vec<Vec<i32>> = vec![history.clone()];
        let mut history = history;
        while !history.iter().all(|n| *n == 0) {
            let mut current = vec![];
            for (a, b) in history.iter().tuple_windows() {
                current.push(b - a);
            }
            vec.push(current.clone());
            history = current;
        }

        let history_value =
            vec.iter().fold(0, |acc, v| acc + v.last().unwrap());
        // println!("{}", history_value);
        sum += history_value;

        let backward =
            vec.iter().rev().fold(0, |acc, v| v.first().unwrap() - acc);
        sum_2 += backward;
    }
    (sum, sum_2)
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_part1() {
        let input = indoc! {
            r#"
            0 3 6 9 12 15
            1 3 6 10 15 21
            10 13 16 21 30 45
            "#
        };
        assert_eq!(process(input), (114, 2))
    }
}
