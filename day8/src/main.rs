use std::collections::HashMap;

use nom::bytes::complete::tag;
use nom::character::complete::alphanumeric1;
use nom::sequence::{delimited, separated_pair};
use nom::IResult;

fn main() {
    let input = include_str!("input.txt");
    println!("{}", part1(input));
    println!("{}", part2(input));
}

enum Instruction {
    Left,
    Right,
}

impl From<char> for Instruction {
    fn from(value: char) -> Self {
        match value {
            'L' => Instruction::Left,
            'R' => Instruction::Right,
            _ => unreachable!(),
        }
    }
}

struct Network<'a> {
    net: HashMap<&'a str, Node<'a>>,
}

impl<'a> Network<'a> {
    fn new(net: &'a str) -> Network<'a> {
        let network = net
            .lines()
            .map(|line| parse_network(line).unwrap().1)
            .map(|n| (n.node_id, n))
            .collect::<HashMap<&str, Node>>();

        Network { net: network }
    }
}

fn find_step<I: Iterator<Item = Instruction>>(
    curr: &str,
    mut ins_iter: I,
    network: &Network,
) -> u64 {
    let mut curr = curr;
    let mut step = 0;
    while !curr.ends_with('Z') {
        let ins = ins_iter
            .next()
            .expect("should have instruction in ins iterator");
        curr = match ins {
            Instruction::Left => network.net.get(curr).unwrap().left,
            Instruction::Right => {
                network.net.get(curr).unwrap().right
            }
        };
        step += 1;
    }

    step
}

fn part1(input: &str) -> u64 {
    let (ins, net) = input
        .split_once("\n\n")
        .expect("should have a empty line in middle");
    let ins_iter = ins.chars().map(Instruction::from).cycle();
    let network = Network::new(net);
    find_step("AAA", Box::new(ins_iter), &network)
}

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn lcm(a: u64, b: u64) -> u64 {
    a * b / gcd(a, b)
}

fn part2(input: &str) -> u64 {
    let (ins, net) = input
        .split_once("\n\n")
        .expect("should have a empty line in middle");
    let ins_iter = ins.chars().map(Instruction::from).cycle();
    let network = Network::new(net);

    network
        .net
        .keys()
        .filter(|node_id| node_id.ends_with('A'))
        .copied()
        .map(|curr| find_step(curr, ins_iter.clone(), &network))
        .reduce(lcm)
        .unwrap()
}

struct Node<'a> {
    node_id: &'a str,
    left: &'a str,
    right: &'a str,
}

fn parse_navigate(input: &str) -> IResult<&str, (&str, &str)> {
    delimited(
        tag("("),
        separated_pair(alphanumeric1, tag(", "), alphanumeric1),
        tag(")"),
    )(input)
}

fn parse_network(input: &str) -> IResult<&str, Node> {
    let (input, (node_id, (left, right))) =
        separated_pair(alphanumeric1, tag(" = "), parse_navigate)(
            input,
        )?;
    Ok((
        input,
        Node {
            node_id,
            left,
            right,
        },
    ))
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    #[test]
    fn test_part1() {
        let input = indoc! {
            r#"
            RL

            AAA = (BBB, CCC)
            BBB = (DDD, EEE)
            CCC = (ZZZ, GGG)
            DDD = (DDD, DDD)
            EEE = (EEE, EEE)
            GGG = (GGG, GGG)
            ZZZ = (ZZZ, ZZZ)
            "#
        };
        assert_eq!(2, part1(input));

        let input = indoc! {
            r#"
            LLR

            AAA = (BBB, BBB)
            BBB = (AAA, ZZZ)
            ZZZ = (ZZZ, ZZZ)
            "#
        };
        assert_eq!(6, part1(input))
    }

    #[test]
    fn test_part2() {
        let input = indoc! {
            r#"
            LR

            11A = (11B, XXX)
            11B = (XXX, 11Z)
            11Z = (11B, XXX)
            22A = (22B, XXX)
            22B = (22C, 22C)
            22C = (22Z, 22Z)
            22Z = (22B, 22B)
            XXX = (XXX, XXX)
            "#
        };

        assert_eq!(part2(input), 6);
    }
}
