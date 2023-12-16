use std::collections::VecDeque;
use std::fmt::{Display, Formatter};

fn main() {
    let input = include_str!("input.txt");
    println!("{:?}", part1(input));
    println!("{:?}", part2(input));
}

fn part1(input: &str) -> u32 {
    input.trim().split(',').map(|s| hash(s, 0)).sum()
}

#[derive(Copy, Clone, Debug)]
struct Step<'a> {
    origin: &'a str,
    label: &'a str,
    focal: &'a str,
}

impl Display for Step<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "[{} {}]", self.label, self.focal)?;
        Ok(())
    }
}

enum StepType<'a> {
    Add(Step<'a>),
    Remove(Step<'a>),
}

fn part2(input: &str) -> usize {
    let mut boxes = vec![VecDeque::<Step<'_>>::new(); 256];
    input
        .trim()
        .split(',')
        .map(|origin| (origin, origin.split_once(['=', '-']).unwrap()))
        .map(|(origin, (label, focal))| {
            let step = Step {
                origin,
                label,
                focal,
            };
            (
                match focal {
                    "" => StepType::Remove(step),
                    _ => StepType::Add(step),
                },
                hash(label, 0) as usize,
            )
        })
        .for_each(|(step_type, box_n)| {
            let tbox = &mut boxes[box_n];
            match step_type {
                // 1. If there is already a lens in the box with the same label,
                // replace the old lens with the new lens: remove the old lens and put the new lens in its place,
                // not moving any other lenses in the box.

                // 2. If there is not already a lens in the box with the same label,
                // add the lens to the box immediately behind any lenses already in the box.
                // Don't move any of the other lenses when you do this.
                // If there aren't any lenses in the box,
                // the new lens goes all the way to the front of the box.
                StepType::Add(step) => {
                    if let Some(pos) =
                        tbox.iter().position(|s| step.label == s.label)
                    {
                        tbox[pos] = step;
                    } else {
                        tbox.push_back(step);
                    }
                }
                StepType::Remove(step) => {
                    if let Some(pos) =
                        tbox.iter().position(|s| step.label == s.label)
                    {
                        tbox.remove(pos);
                    }
                }
            }
        });
    boxes
        .iter()
        .enumerate()
        .map(|(b_index, b)| {
            b.iter()
                .enumerate()
                .map(|(index, step)| {
                    step.focal.parse::<usize>().expect("should be a number")
                        * (index + 1)
                        * (b_index + 1)
                })
                .sum::<usize>()
        })
        .sum::<usize>()
}

fn hash(input: &str, value: u32) -> u32 {
    input
        .chars()
        .fold(value, |acc, ch| ((ch as u32 + acc) * 17) % 256)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash() {
        assert_eq!(hash("HASH", 0), 52);
    }

    #[test]
    fn test_part1() {
        let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
        assert_eq!(part1(input), 1320);
    }

    #[test]
    fn test_part2() {
        let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
        assert_eq!(part2(input), 145);
    }
}
