use cached::proc_macro::cached;
use std::iter::repeat;

fn main() {
    let input = include_str!("input.txt");
    println!("{}", part1(input));
    println!("{}", part2(input));
}

fn part1(input: &str) -> usize {
    input.lines().map(find_arrangements).sum()
}

fn part2(input: &str) -> usize {
    input.lines().map(find_arrangement_with_repeat).sum()
}

fn find_arrangement_with_repeat(line: &str) -> usize {
    let (springs, numbers) = line.split_once(' ').unwrap();
    let (springs, numbers) = (
        repeat(springs).take(5).collect::<Vec<&str>>().join("?"),
        repeat(numbers).take(5).collect::<Vec<&str>>().join(","),
    );
    let (springs, numbers) = (
        springs.chars().collect(),
        numbers
            .split(',')
            .map(|s| s.parse::<usize>().unwrap())
            .collect::<Vec<usize>>(),
    );
    cached_count(springs, numbers)
}

#[cached]
fn cached_count(springs: Vec<char>, numbers: Vec<usize>) -> usize {
    if numbers.is_empty() {
        return if springs.contains(&'#') { 0 } else { 1 };
    }
    if springs.is_empty() {
        return if numbers.is_empty() { 1 } else { 0 };
    }

    let mut res = 0;
    if ".?".contains(springs[0]) {
        res += cached_count(
            springs.get(1..).unwrap_or_default().to_vec(),
            numbers.clone(),
        )
    }
    if "#?".contains(springs[0])
        && numbers[0] <= springs.len()
        && !springs[..numbers[0]].contains(&'.')
        && (springs.len() == numbers[0] || springs[numbers[0]] != '#')
    {
        res += cached_count(
            springs.get(numbers[0] + 1..).unwrap_or_default().to_vec(),
            numbers.get(1..).unwrap_or_default().to_vec().to_vec(),
        )
    }
    res
}

fn find_arrangements(line: &str) -> usize {
    let (springs, numbers) = line.split_once(' ').unwrap();
    let numbers = numbers
        .split(',')
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();
    let springs = springs.chars().collect();

    cached_count(springs, numbers)
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case("???.### 1,1,3", 1)]
    #[case(".??..??...?##. 1,1,3", 4)]
    #[case("?#?#?#?#?#?#?#? 1,3,1,6", 1)]
    #[case("????.#...#... 4,1,1", 1)]
    #[case("????.######..#####. 1,6,5", 4)]
    #[case("?###???????? 3,2,1", 10)]
    fn test_part1(#[case] line: &str, #[case] expected: usize) {
        assert_eq!(find_arrangements(line), expected);
    }

    #[rstest]
    #[case("???.### 1,1,3", 1)]
    #[case(".??..??...?##. 1,1,3", 16384)]
    #[case("?#?#?#?#?#?#?#? 1,3,1,6", 1)]
    #[case("????.#...#... 4,1,1", 16)]
    #[case("????.######..#####. 1,6,5", 2500)]
    #[case("?###???????? 3,2,1", 506250)]
    fn test_part2(#[case] line: &str, #[case] expected: usize) {
        assert_eq!(find_arrangement_with_repeat(line), expected);
    }
}
