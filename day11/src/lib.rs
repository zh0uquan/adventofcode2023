use std::collections::{HashMap, HashSet};
use std::fmt::{Display, Formatter};
use std::hash::Hash;

use itertools::Itertools;
use pathfinding::prelude::astar;

pub fn process(input: &str, expand_size: usize) -> usize {
    let universe = Universe::new(input);
    // println!("{}", universe);
    let edges = universe.edges(expand_size);
    universe
        .get_pairs()
        .map(|pair| universe.a_star_distance(pair, &edges))
        .sum()
}

pub fn process_manhattan(input: &str, expand_size: usize) -> usize {
    let universe = Universe::new(input);
    // println!("{}", universe);
    let edges = universe.edges(expand_size);
    universe
        .get_pairs()
        .map(|pair| universe.manhattan_distance(pair, expand_size))
        .sum()
}

type Coord = (usize, usize);
type Edge = (Coord, Coord);

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
struct Galaxy {
    m: usize,
    n: usize,
    id: usize,
}

impl Galaxy {
    fn coord(&self) -> Coord {
        (self.m, self.n)
    }
}

struct Universe {
    grid: Vec<Vec<char>>,
    height: usize,
    width: usize,
}

impl Universe {
    fn new(input: &str) -> Universe {
        let grid: Vec<Vec<char>> = input
            .lines()
            .map(|line| line.chars().collect())
            .collect();
        let height = grid.len();
        let width = grid[0].len();
        Universe {
            grid,
            height,
            width,
        }
    }

    fn edges(&self, with_expand_size: usize) -> HashMap<Edge, usize> {
        let (rows_indexes, cols_indexes) = self.get_empty_indexes();
        let start = Coord::default();
        let mut stack = vec![start];
        let mut edges = HashMap::new();
        let mut visited = HashSet::new();

        while !stack.is_empty() {
            let mut new_stack = vec![];
            while let Some(node) = stack.pop() {
                for neighbour in self.get_neighbours(node) {
                    if visited.contains(&(node, neighbour))
                        || visited.contains(&(neighbour, node))
                    {
                        continue;
                    }
                    visited.insert((node, neighbour));
                    visited.insert((neighbour, node));

                    let mut distance = 1;
                    if rows_indexes.contains(&(neighbour.0))
                        || cols_indexes.contains(&(neighbour.1))
                    {
                        distance = with_expand_size;
                    }
                    edges.insert((node, neighbour), distance);
                    edges.insert((neighbour, node), distance);

                    new_stack.push(neighbour);
                }
            }
            stack = new_stack;
        }
        edges
    }

    /// Get the row and cols indexes where lines only have '.' char.
    fn get_empty_indexes(&self) -> (Vec<usize>, Vec<usize>) {
        let mut rows_indexes: Vec<usize> = vec![];
        let mut cols_indexes: Vec<usize> = vec![];

        for m in 0..self.height {
            if self.grid[m].iter().all(|c| *c == '.') {
                rows_indexes.push(m);
            }
        }
        for n in 0..self.width {
            if (0..self.height)
                .map(|m| self.grid[m][n])
                .all(|c| c == '.')
            {
                cols_indexes.push(n);
            }
        }

        (rows_indexes, cols_indexes)
    }

    fn expand(&mut self) -> Universe {
        let (rows_indexes, cols_indexes) = self.get_empty_indexes();
        rows_indexes.into_iter().enumerate().for_each(|(i, m)| {
            self.grid.insert(m + i, self.grid[m + i].clone())
        });

        let height = self.grid.len();
        cols_indexes.into_iter().enumerate().for_each(|(i, n)| {
            (0..height).for_each(|m| self.grid[m].insert(n + i, '.'))
        });
        let width = self.grid[0].len();

        Universe {
            grid: self.grid.clone(),
            width,
            height,
        }
    }

    fn get_pairs(
        &self,
    ) -> impl Iterator<Item = (Galaxy, Galaxy)> + '_ {
        self.grid
            .iter()
            .enumerate()
            .flat_map(|(m, row)| {
                row.iter()
                    .enumerate()
                    .filter(|(_, ch)| **ch == '#')
                    .map(move |(n, _)| (m, n))
            })
            .enumerate()
            .map(|(id, (m, n))| Galaxy { m, n, id: id + 1 })
            .tuple_combinations()
    }

    fn get_neighbours(&self, node: Coord) -> Vec<Coord> {
        [(0, 1), (1, 0), (0, -1), (-1, 0)]
            .into_iter()
            .map(|(dx, dy)| {
                (
                    node.0.saturating_add_signed(dx),
                    node.1.saturating_add_signed(dy),
                )
            })
            .filter(|coord| {
                *coord != node
                    && coord.0 < self.height
                    && coord.1 < self.width
            })
            .collect()
    }

    fn manhattan_distance(
        &self,
        pair: (Galaxy, Galaxy),
        expand_size: usize,
    ) -> usize {
        let (start, end) = (pair.0.coord(), pair.1.coord());
        let (rows_indexes, cols_indexes) = self.get_empty_indexes();

        let a: usize = (start.0.min(end.0)..start.0.max(end.0))
            .map(|r| {
                if rows_indexes.contains(&r) {
                    return expand_size;
                }
                1
            })
            .sum();
        let b: usize = (start.1.min(end.1)..start.1.max(end.1))
            .map(|c| {
                if cols_indexes.contains(&c) {
                    return expand_size;
                }
                1
            })
            .sum();
        a + b
    }

    fn a_star_distance(
        &self,
        pair: (Galaxy, Galaxy),
        edges: &HashMap<Edge, usize>,
    ) -> usize {
        let (start, end) = (pair.0.coord(), pair.1.coord());

        let heuristic = |node: &Coord| -> usize {
            node.0.abs_diff(end.0) + node.1.abs_diff(end.1)
        };

        astar(
            &start,
            |n| {
                self.get_neighbours(*n)
                    .iter()
                    .map(|neighbour| {
                        (*neighbour, edges[&(*neighbour, *n)])
                    })
                    .collect::<Vec<(Coord, usize)>>()
            },
            heuristic,
            |n| *n == end,
        )
        .expect("should have a distance")
        .1
    }
}

impl Display for Universe {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for row in self.grid.iter() {
            writeln!(f, "{}", row.iter().collect::<String>())?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use indoc::indoc;
    use rstest::{fixture, rstest};

    use super::*;

    #[fixture]
    fn input() -> &'static str {
        return indoc! {
            r"
            ...#......
            .......#..
            #.........
            ..........
            ......#...
            .#........
            .........#
            ..........
            .......#..
            #...#.....
            "
        };
    }

    #[fixture]
    fn universe(input: &str) -> Universe {
        Universe::new(input)
    }

    #[rstest]
    fn test_expand(universe: Universe) {
        let mut universe = universe;
        universe.expand();

        let expanded_uni = indoc! {
            r"
            ....#........
            .........#...
            #............
            .............
            .............
            ........#....
            .#...........
            ............#
            .............
            .............
            .........#...
            #....#.......
            "
        };

        assert_eq!(universe.to_string(), expanded_uni);
    }

    #[rstest]
    fn test_part1(input: &str) {
        let universe = Universe::new(input);
        let edges = universe.edges(2);
        universe.get_pairs().for_each(|(g1, g2)| {
            if g1.id == 3 && g2.id == 6 {
                assert_eq!(
                    17,
                    universe.a_star_distance((g1, g2), &edges)
                );
            }
        });

        assert_eq!(374, process(input, 2));
        assert_eq!(374, process_manhattan(input, 2));
    }

    #[rstest]
    fn test_part2(input: &str) {
        assert_eq!(process(input, 2), 374);
        assert_eq!(process(input, 10), 1030);
        assert_eq!(process(input, 100), 8410);
        assert_eq!(8410, process_manhattan(input, 100));
    }
}
