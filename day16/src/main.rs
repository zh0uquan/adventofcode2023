use derive_more::Display;
use std::collections::{HashMap, HashSet};
use std::fmt::{Display, Formatter};

fn main() {
    let input = include_str!("input.txt");
    println!("{:?}", part1(input));
    println!("{:?}", part2(input));
}

fn part1(input: &str) -> usize {
    let mut state = State::new(
        input,
        Light {
            curr: START_COORD,
            direction: Direction::East,
        },
    );
    state.tick();
    state
        .visited
        .iter()
        .map(|v| v.0)
        .collect::<HashSet<Coord>>()
        .len()
        - 1
}

fn part2(input: &str) -> usize {
    let height = input.lines().next().unwrap().len() as isize;
    let width = input.lines().count() as isize;
    let rows: Vec<Light> = (0..height)
        .flat_map(|m| {
            vec![
                Light {
                    curr: (m, -1),
                    direction: Direction::East,
                },
                Light {
                    curr: (m, width),
                    direction: Direction::West,
                },
            ]
        })
        .collect();
    let cols: Vec<Light> = (0..width)
        .flat_map(|n| {
            vec![
                Light {
                    curr: (-1, n),
                    direction: Direction::South,
                },
                Light {
                    curr: (height, n),
                    direction: Direction::North,
                },
            ]
        })
        .collect();

    rows.iter()
        .chain(&cols)
        .map(|light| {
            let mut state = State::new(input, *light);
            state.tick();
            state
                .visited
                .iter()
                .map(|v| v.0)
                .collect::<HashSet<Coord>>()
                .len()
                - 1
        })
        .max()
        .unwrap()
}

type Coord = (isize, isize);

const START_COORD: (isize, isize) = (0, -1);

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
enum Direction {
    North,
    South,
    West,
    East,
}

impl Direction {
    fn forward(&self) -> (isize, isize) {
        match self {
            Direction::North => (-1, 0),
            Direction::South => (1, 0),
            Direction::West => (0, -1),
            Direction::East => (0, 1),
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct Light {
    curr: Coord,
    direction: Direction,
}

impl Light {
    fn next_possible_pos(&self) -> Coord {
        let forward = self.direction.forward();
        let curr_pos = self.curr;
        (curr_pos.0 + forward.0, curr_pos.1 + forward.1)
    }

    fn step(&mut self, state: &mut State) -> Option<Light> {
        let next = self.next_possible_pos();
        if !state.grid.contains_key(&next) {
            return None;
        }

        self.curr = next;
        match state.grid.get(&next).unwrap() {
            Tile::Empty(_) => {
                return None;
            }
            Tile::Mirror(mirror) => match (mirror, self.direction) {
                ('|', Direction::East | Direction::West) => {
                    self.direction = Direction::North;
                    return Some(Light {
                        curr: next,
                        direction: Direction::South,
                    });
                }
                ('-', Direction::South | Direction::North) => {
                    self.direction = Direction::East;
                    return Some(Light {
                        curr: next,
                        direction: Direction::West,
                    });
                }
                ('/', Direction::East) => self.direction = Direction::North,
                ('/', Direction::West) => self.direction = Direction::South,
                ('/', Direction::North) => self.direction = Direction::East,
                ('/', Direction::South) => self.direction = Direction::West,

                ('\\', Direction::East) => self.direction = Direction::South,
                ('\\', Direction::West) => self.direction = Direction::North,
                ('\\', Direction::North) => self.direction = Direction::West,
                ('\\', Direction::South) => self.direction = Direction::East,
                _ => (),
            },
        }
        None
    }
}

#[derive(Debug, Display)]
enum Tile {
    #[display(fmt = "{}", _0)]
    Empty(char),
    #[display(fmt = "{}", _0)]
    Mirror(char),
}

#[derive(Debug)]
struct State {
    grid: HashMap<Coord, Tile>,
    visited: HashSet<(Coord, Direction)>,
    lights: Vec<Light>,
    height: isize,
    width: isize,
}

impl Display for State {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let seen: HashSet<Coord> = self.visited.iter().map(|v| v.0).collect();
        for m in 0..self.height {
            for n in 0..self.width {
                if seen.contains(&(m, n)) {
                    write!(f, "#")?;
                } else {
                    write!(f, "{}", self.grid.get(&(m, n)).unwrap())?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl State {
    fn new(input: &str, start_light: Light) -> State {
        let height = input.lines().next().unwrap().len() as isize;
        let width = input.lines().count() as isize;
        let grid = input
            .lines()
            .enumerate()
            .flat_map(|(m, line)| {
                line.chars().enumerate().map(move |(n, ch)| {
                    (
                        (m as isize, n as isize),
                        match ch {
                            '.' => Tile::Empty('.'),
                            mirror @ ('/' | '|' | '-' | '\\') => {
                                Tile::Mirror(mirror)
                            }
                            _ => panic!("lava disco!"),
                        },
                    )
                })
            })
            .collect();

        State {
            grid,
            lights: vec![start_light],
            visited: HashSet::new(),
            height,
            width,
        }
    }

    fn tick(&mut self) {
        while !self.lights.is_empty() {
            let mut new_lights = vec![];
            while let Some(mut light) = self.lights.pop() {
                if self.visited.contains(&(light.curr, light.direction)) {
                    continue;
                }
                self.visited.insert((light.curr, light.direction));
                if let Some(new_light) = light.step(self) {
                    new_lights.push(new_light);
                }
                new_lights.push(light);
            }
            self.lights = new_lights;
            // println!("{:?} {:?}", self.lights, self.visited);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_part1() {
        let input = indoc! {
            r#"
            .|...\....
            |.-.\.....
            .....|-...
            ........|.
            ..........
            .........\
            ..../.\\..
            .-.-/..|..
            .|....-|.\
            ..//.|....
            "#
        };
        assert_eq!(part1(input), 46);
    }

    #[test]
    fn test_part2() {
        let input = indoc! {
            r#"
            .|...\....
            |.-.\.....
            .....|-...
            ........|.
            ..........
            .........\
            ..../.\\..
            .-.-/..|..
            .|....-|.\
            ..//.|....
            "#
        };
        assert_eq!(part2(input), 51);
    }
}
