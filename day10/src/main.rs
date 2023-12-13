use std::collections::{HashMap, HashSet};

fn main() {
    let input = include_str!("input.txt");
    println!("{}", part1(input));
    println!("{}", part2(input));
}

fn visualize_part1(maze: &Maze, distance: &HashMap<Coord, u32>) {
    for m in 0..=maze.height {
        let mut string = String::new();
        for n in 0..=maze.width {
            if distance.contains_key(&(m, n))
                && *distance.get(&(m, n)).unwrap() < 10
            {
                string.push(
                    char::from_digit(*distance.get(&(m, n)).unwrap(), 10)
                        .unwrap(),
                );
            } else {
                string.push(maze.grid.get(&(m, n)).unwrap().form);
            }
        }
        println!("{}", string);
    }
}

fn part1(input: &str) -> usize {
    let maze = Maze::new(input);
    let start = maze.start;
    let mut distance: HashMap<Coord, u32> = HashMap::new();
    let mut stack = vec![&start];
    distance.insert(start.coord, 0);

    while !stack.is_empty() {
        let mut new_stack = vec![];
        while let Some(pipe) = stack.pop() {
            for neighbour in pipe.neighhours(&maze) {
                // println!("{:?}: {:?}", pipe, neighbour);
                if distance.contains_key(&neighbour.coord) {
                    continue;
                }
                distance.insert(
                    neighbour.coord,
                    distance.get(&pipe.coord).unwrap() + 1,
                );
                new_stack.push(neighbour)
            }
        }
        stack = new_stack;
    }

    *distance.values().max().unwrap() as usize
}

fn ray_check_inside(
    coord: &Coord,
    seen: &HashSet<Coord>,
    maze: &Maze,
) -> bool {
    // ray check from 0 to the coord
    let mut count = 0;
    let mut y = coord.1;
    while let Some(ny) = y.checked_sub(1) {
        let ch = maze.grid.get(&(coord.0, ny)).unwrap().form;
        if seen.contains(&(coord.0, ny)) && "|LJS".contains(ch) {
            count += 1;
        }
        y = ny;
    }
    count % 2 != 0
}

fn part2(input: &str) -> usize {
    let maze = Maze::new(input);
    let start = maze.start;
    let mut seen = HashSet::new();
    let mut stack = vec![&start];
    seen.insert(start.coord);

    while !stack.is_empty() {
        let mut new_stack = vec![];
        while let Some(pipe) = stack.pop() {
            for neighbour in pipe.neighhours(&maze) {
                // println!("{:?}: {:?}", pipe, neighbour);
                if seen.contains(&neighbour.coord) {
                    continue;
                }
                seen.insert(neighbour.coord);
                new_stack.push(neighbour)
            }
        }
        stack = new_stack;
    }

    let mut row_min_max_map: HashMap<usize, (usize, usize)> = HashMap::new();
    for (m, n) in seen.iter() {
        row_min_max_map
            .entry(*m)
            .and_modify(|(min, max)| {
                *min = *min.min(&mut n.clone());
                *max = *max.max(&mut n.clone())
            })
            .or_insert((*n, *n));
    }

    let mut count = 0;
    for (m, n) in maze.grid.keys() {
        let (min, max) = row_min_max_map.get(m).unwrap_or(&(0, 0));
        if n < max
            && n > min
            && !seen.contains(&(*m, *n))
            && ray_check_inside(&(*m, *n), &seen, &maze)
        {
            count += 1;
        }
    }

    count
}

type Coord = (usize, usize);

#[derive(Default, Eq, PartialEq, Hash, Clone, Copy, Debug)]
struct Pipe {
    coord: Coord,
    form: char,
}

#[derive(Copy, Clone)]
enum Direction {
    North,
    West,
    East,
    South,
}

impl Pipe {
    fn get_coord(self, direction: Direction) -> Coord {
        match direction {
            Direction::North => (self.coord.0.saturating_sub(1), self.coord.1),
            Direction::West => (self.coord.0, self.coord.1.saturating_sub(1)),
            Direction::East => (self.coord.0, self.coord.1.saturating_add(1)),
            Direction::South => (self.coord.0.saturating_add(1), self.coord.1),
        }
    }

    fn neighhours<'a>(&self, maze: &'a Maze) -> Vec<&'a Pipe> {
        let coords: Vec<Coord> = match self.form {
            '|' => {
                vec![
                    self.get_coord(Direction::South),
                    self.get_coord(Direction::North),
                ]
            }
            '-' => {
                vec![
                    self.get_coord(Direction::East),
                    self.get_coord(Direction::West),
                ]
            }
            'L' => {
                vec![
                    self.get_coord(Direction::North),
                    self.get_coord(Direction::East),
                ]
            }
            'J' => {
                vec![
                    self.get_coord(Direction::North),
                    self.get_coord(Direction::West),
                ]
            }
            '7' => {
                vec![
                    self.get_coord(Direction::South),
                    self.get_coord(Direction::West),
                ]
            }
            'F' => {
                vec![
                    self.get_coord(Direction::South),
                    self.get_coord(Direction::East),
                ]
            }
            'S' => return maze.get_start_neighbours(),
            _ => panic!("disco!"),
        };
        coords.iter().filter_map(|c| maze.grid.get(c)).collect()
    }
}

struct Maze {
    grid: HashMap<Coord, Pipe>,
    start: Pipe,
    height: usize,
    width: usize,
}

impl Maze {
    fn new(input: &str) -> Maze {
        let input = input.lines();
        let mut grid = HashMap::new();
        let mut start: Pipe = Pipe::default();
        let mut width = 0;
        let mut height = 0;
        for (m, row) in input.enumerate() {
            for (n, ch) in row.chars().enumerate() {
                let pipe = Pipe {
                    coord: (m, n),
                    form: ch,
                };
                grid.insert((m, n), pipe);
                if ch == 'S' {
                    start = Pipe {
                        coord: (m, n),
                        form: ch,
                    }
                }
                width = width.max(n);
            }
            height = height.max(m);
        }
        Maze {
            grid,
            start,
            width,
            height,
        }
    }

    fn get_start_neighbours(&self) -> Vec<&Pipe> {
        let start = self.start;
        let mut start_neighbours = vec![];

        for direction in [
            Direction::North,
            Direction::West,
            Direction::East,
            Direction::South,
        ] {
            let pipe = self.grid.get(&start.get_coord(direction)).unwrap();
            match direction {
                Direction::North => match pipe.form {
                    '|' | '7' | 'F' => start_neighbours.push(pipe),
                    _ => (),
                },
                Direction::West => match pipe.form {
                    '-' | 'F' | 'L' => start_neighbours.push(pipe),
                    _ => (),
                },
                Direction::East => match pipe.form {
                    '-' | 'J' | '7' => start_neighbours.push(pipe),
                    _ => (),
                },
                Direction::South => match pipe.form {
                    '|' | 'J' | 'L' => start_neighbours.push(pipe),
                    _ => (),
                },
            }
        }
        start_neighbours
    }
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    #[test]
    fn test_part1() {
        let input = indoc! {
            ".....
            .S-7.
            .|.|.
            .L-J.
            ....."
        };
        assert_eq!(part1(input), 4);

        let input = indoc! {
            r#"
            ..F7.
            .FJ|.
            SJ.L7
            |F--J
            LJ...
            "#
        };
        assert_eq!(part1(input), 8);
    }

    #[test]
    fn test_part2() {
        let input = indoc! {
            r#"
            ...........
            .S-------7.
            .|F-----7|.
            .||.....||.
            .||.....||.
            .|L-7.F-J|.
            .|..|.|..|.
            .L--J.L--J.
            ...........
            "#
        };
        assert_eq!(4, part2(input));

        let input = indoc! {
            r"
            .F----7F7F7F7F-7....
            .|F--7||||||||FJ....
            .||.FJ||||||||L7....
            FJL7L7LJLJ||LJ.L-7..
            L--J.L7...LJS7F-7L7.
            ....F-J..F7FJ|L7L7L7
            ....L7.F7||L7|.L7L7|
            .....|FJLJ|FJ|F7|.LJ
            ....FJL-7.||.||||...
            ....L---J.LJ.LJLJ...
            "
        };
        assert_eq!(8, part2(input));

        let input = indoc! {
            r"
            FF7FSF7F7F7F7F7F---7
            L|LJ||||||||||||F--J
            FL-7LJLJ||||||LJL-77
            F--JF--7||LJLJ7F7FJ-
            L---JF-JLJ.||-FJLJJ7
            |F|F-JF---7F7-L7L|7|
            |FFJF7L7F-JF7|JL---7
            7-L-JL7||F7|L7F-7F7|
            L.L7LFJ|||||FJL7||LJ
            L7JLJL-JLJLJL--JLJ.L
            "
        };

        assert_eq!(10, part2(input));
    }
}
