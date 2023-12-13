use std::iter::zip;

fn main() {
    let input = include_str!("input.txt");
    println!("{:?}", process(input, 0));
    println!("{:?}", process(input, 1));
}

fn process(input: &str, smudge: usize) -> usize {
    input
        .split("\n\n")
        .map(Matrix::new)
        .map(|m| m.find_mirror(smudge))
        .sum()
}

#[derive(Debug)]
struct Matrix {
    rows: Vec<Vec<u8>>,
    cols: Vec<Vec<u8>>,
}

impl Matrix {
    fn new(input: &str) -> Matrix {
        let rows: Vec<Vec<u8>> =
            input.lines().map(|l| l.as_bytes().to_vec()).collect();
        let height = rows.len();
        let width = rows[0].len();

        let cols: Vec<Vec<u8>> = (0..width)
            .map(|n| (0..height).map(|m| rows[m][n]).collect())
            .collect();

        Matrix { rows, cols }
    }

    fn find_mirror(&self, smudge_n: usize) -> usize {
        let find_index = |lines: &Vec<Vec<u8>>| -> Option<usize> {
            (0..lines.len() - 1)
                .position(|index| {
                    let mut smudge_n = smudge_n;
                    let is_match = zip(
                        lines[..index + 1].iter().rev(),
                        lines[index + 1..].iter(),
                    )
                    .all(|(l1, l2)| {
                        if l1 == l2 {
                            return true;
                        }
                        if smudge_n == 0 {
                            return false;
                        }
                        zip(l1, l2).all(|(c1, c2)| {
                            if c1 == c2 {
                                return true;
                            }
                            if smudge_n == 0 {
                                return false;
                            }
                            smudge_n -= 1;
                            if c1.abs_diff(*c2) == b'#'.abs_diff(b'.') {
                                return true;
                            }
                            false
                        })
                    });
                    if smudge_n == 0 && is_match {
                        return true;
                    }
                    false
                })
                .map(|index| index + 1)
        };
        find_index(&self.rows)
            .map(|c| c * 100)
            .or(find_index(&self.cols))
            .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    #[test]
    fn test_process() {
        let input = indoc! {
            r#"
            #.##..##.
            ..#.##.#.
            ##......#
            ##......#
            ..#.##.#.
            ..##..##.
            #.#.##.#.
            "#
        };

        let matrix = Matrix::new(input);
        assert_eq!(matrix.find_mirror(0), 5);
        assert_eq!(matrix.find_mirror(1), 300);

        let input = indoc! {
            r#"
            #...##..#
            #....#..#
            ..##..###
            #####.##.
            #####.##.
            ..##..###
            #....#..#
            "#
        };
        let matrix = Matrix::new(input);
        assert_eq!(matrix.find_mirror(0), 400);
        assert_eq!(matrix.find_mirror(1), 100);
    }
}
