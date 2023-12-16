use std::collections::HashMap;
use std::fmt::{Display, Formatter};

pub fn part1(input: &str) -> usize {
    let mut matrix = Matrix::new(input);
    let coords =
        get_matrix_coords(matrix.width, matrix.height, Direction::North);
    tilt_matrix(&coords, &mut matrix.matrix);
    calculate_load(&matrix.matrix)
}

pub fn part2(input: &str) -> usize {
    let mut matrix = Matrix::new(input);
    let cycle = [
        Direction::North,
        Direction::West,
        Direction::South,
        Direction::East,
    ]
    .map(|d| (d, get_matrix_coords(matrix.width, matrix.height, d)));

    let mut cached: HashMap<(String, Direction), usize> = HashMap::new();
    let mut i = 0;
    let mut repeat = 0;
    let mut start = 0;

    'outer: while i <= 1000000000 {
        for (direction, coords) in cycle.iter() {
            tilt_matrix(coords, &mut matrix.matrix);
            if cached.contains_key(&(matrix.to_string(), *direction)) {
                start = cached[&(matrix.to_string(), *direction)];
                repeat = i - start;
                break 'outer;
            }
            cached.insert((matrix.to_string(), *direction), i);
        }
        i += 1;
    }

    let mut matrix = Matrix::new(input);
    for _ in 0..(1000000000 - start) % repeat + start {
        for (_direction, coords) in cycle.iter() {
            tilt_matrix(coords, &mut matrix.matrix);
        }
    }

    calculate_load(&matrix.matrix)
}

#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug)]
enum Direction {
    North,
    West,
    East,
    South,
}

#[derive(Debug, Clone)]
struct Matrix {
    matrix: Vec<Vec<char>>,
    width: usize,
    height: usize,
}

impl Display for Matrix {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for row in self.matrix.iter() {
            writeln!(f, "{}", row.iter().collect::<String>())?;
        }
        Ok(())
    }
}

type Coord = (usize, usize);

impl Matrix {
    fn new(input: &str) -> Matrix {
        let width = input.lines().next().unwrap().len();
        let height = input.lines().count();
        let matrix = input
            .lines()
            .map(|l| l.chars().collect::<Vec<char>>())
            .collect();
        Matrix {
            height,
            width,
            matrix,
        }
    }
}

fn get_matrix_coords(
    width: usize,
    height: usize,
    direction: Direction,
) -> Vec<Vec<Coord>> {
    match direction {
        Direction::North => (0..width)
            .map(|n| (0..height).map(move |m| (m, n)).collect::<Vec<Coord>>())
            .collect(),
        Direction::West => (0..height)
            .map(|m| (0..width).map(move |n| (m, n)).collect::<Vec<Coord>>())
            .collect(),
        Direction::East => (0..height)
            .map(|m| {
                (0..width)
                    .rev()
                    .map(move |n| (m, n))
                    .collect::<Vec<Coord>>()
            })
            .collect(),
        Direction::South => (0..width)
            .map(|n| {
                (0..height)
                    .rev()
                    .map(move |m| (m, n))
                    .collect::<Vec<Coord>>()
            })
            .collect(),
    }
}

fn calculate_load(matrix: &Vec<Vec<char>>) -> usize {
    matrix
        .iter()
        .rev()
        .enumerate()
        .map(|(i, line)| line.iter().filter(|n| **n == 'O').count() * (i + 1))
        .sum()
}

fn tilt_matrix(coords_vecs: &Vec<Vec<Coord>>, matrix: &mut [Vec<char>]) {
    for line in coords_vecs {
        let mut i = 0;
        while i < line.len() {
            let (m_i, n_i) = line[i];
            if matrix[m_i][n_i] == 'O' || matrix[m_i][n_i] == '#' {
                i += 1;
                continue;
            } else {
                let mut j = i + 1;
                while j < line.len() {
                    let (m_j, n_j) = line[j];

                    if matrix[m_j][n_j] == 'O' {
                        let tmp = matrix[m_i][n_i];
                        matrix[m_i][n_i] = matrix[m_j][n_j];
                        matrix[m_j][n_j] = tmp;
                        break;
                    } else if matrix[m_j][n_j] == '.' {
                        j += 1;
                        continue;
                    } else if matrix[m_j][n_j] == '#' {
                        i = j;
                        break;
                    }
                }
            }
            i += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2};
    use indoc::indoc;

    #[test]
    fn test_part1() {
        let input = indoc! {
            r#"
            O....#....
            O.OO#....#
            .....##...
            OO.#O....O
            .O.....O#.
            O.#..O.#.#
            ..O..#O..O
            .......O..
            #....###..
            #OO..#....
            "#
        };
        assert_eq!(part1(input), 136);
    }

    #[test]
    fn test_part2() {
        let input = indoc! {
            r#"
            O....#....
            O.OO#....#
            .....##...
            OO.#O....O
            .O.....O#.
            O.#..O.#.#
            ..O..#O..O
            .......O..
            #....###..
            #OO..#....
            "#
        };
        assert_eq!(part2(input), 64);
    }
}
