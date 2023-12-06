use std::ops::Range;

use indicatif::ProgressIterator;
use itertools::Itertools;
use nom::bytes::complete::tag;
use nom::character::complete::{
    alpha1, line_ending, space1, u64 as nom_u64,
};
use nom::combinator::opt;
use nom::error::Error;
use nom::multi::{many1, separated_list1};
use nom::sequence::{
    delimited, pair, preceded, separated_pair, terminated,
};
use nom::IResult;

fn find_min<T: ExactSizeIterator<Item = u64>>(
    it: T,
    maps: &[Map],
) -> u64 {
    it.map(|n| {
        maps.iter().fold(n, |mut acc, m| {
            acc = m.convert(acc);
            acc
        })
    })
    .min()
    .unwrap()
}

pub fn part1(input: &str) -> u64 {
    let (_, (seeds, maps)) = parse_garden(input).unwrap();
    find_min(seeds.into_iter(), &maps)
}

pub fn part2_brute_force(input: &str) -> u64 {
    let (_, (seeds, maps)) = parse_garden(input).unwrap();
    let seeds: Vec<u64> = seeds
        .into_iter()
        .tuples()
        .flat_map(|t: (u64, u64)| t.0..t.1 + t.0)
        .collect();
    seeds
        .into_iter()
        .progress()
        .map(|n| {
            maps.iter().fold(n, |mut acc, m| {
                acc = m.convert(acc);
                acc
            })
        })
        .min()
        .unwrap()
}

pub fn part2(input: &str) -> u64 {
    let (_, (seeds, maps)) = parse_garden(input).unwrap();
    seeds
        .into_iter()
        .tuples()
        .map(|t: (u64, u64)| t.0..t.1 + t.0)
        .flat_map(|r| {
            maps.iter().fold(vec![r], |acc, m| {
                let mut new_acc = vec![];
                for r in acc {
                    new_acc.extend(m.convert_range_v2(r));
                }
                new_acc
            })
        })
        .sorted_by(|r1, r2| r1.start.cmp(&r2.start))
        .next()
        .unwrap()
        .start
}

#[derive(Debug)]
struct Map<'a> {
    src: &'a str,
    dst: &'a str,
    range_maps: Vec<RangeMap>,
}

impl<'a> Map<'a> {
    fn convert(&self, n: u64) -> u64 {
        self.range_maps
            .iter()
            .filter_map(|m| m.convert(n))
            .nth(0)
            .unwrap_or(n)
    }

    fn convert_range_v2(&self, r: Range<u64>) -> Vec<Range<u64>> {
        let mut converted = vec![];
        let mut unchanged = vec![];
        let (mut start, end) = (r.start, r.end);

        let convert_ranges: Vec<(Range<u64>, Range<u64>)> = self
            .range_maps
            .iter()
            .filter_map(|m| m.convert_range(r.clone()))
            .sorted_by(|r1, r2| r1.0.start.cmp(&r2.0.start))
            .collect();

        if convert_ranges.is_empty() {
            return vec![r];
        }

        for (src_r, dst_r) in convert_ranges.iter() {
            converted.push(dst_r.clone());
            if src_r.start > start {
                unchanged.push(start..src_r.start);
            }
            start = src_r.end;
        }
        // end case
        let (src_r, _) = convert_ranges.last().unwrap();
        if src_r.end < end {
            unchanged.push(src_r.end..end);
        }
        converted.extend(unchanged);
        converted
    }
}

#[derive(Debug, PartialEq)]
struct RangeMap {
    src_range: Range<u64>,
    dst_range: Range<u64>,
}

impl RangeMap {
    fn convert(&self, n: u64) -> Option<u64> {
        if self.src_range.contains(&n) {
            return Some(
                (n - self.src_range.start) + self.dst_range.start,
            );
        }
        None
    }

    fn convert_range(
        &self,
        r: Range<u64>,
    ) -> Option<(Range<u64>, Range<u64>)> {
        // r: (start, end)
        //    (dst_start, dst_end)
        if (r.start).max(self.src_range.start)
            <= (r.end).min(self.src_range.end)
        {
            let start = if self.src_range.start >= r.start {
                self.src_range.start
            } else {
                r.start
            };
            let end = if self.src_range.end <= r.end {
                self.src_range.end
            } else {
                r.end
            };
            let dst_start =
                start - self.src_range.start + self.dst_range.start;
            let dst_end =
                end + self.dst_range.end - self.src_range.end;
            return Some((start..end, dst_start..dst_end));
        }
        None
    }
}

fn parse_garden(input: &str) -> IResult<&str, (Vec<u64>, Vec<Map>)> {
    let (input, seeds): (&str, Vec<u64>) = delimited(
        tag("seeds: "),
        separated_list1(space1, nom_u64),
        line_ending,
    )(input)?;

    let (input, output) = many1(parse_map)(input)?;
    Ok((input, (seeds, output)))
}

fn parse_map_title(
    input: &str,
) -> IResult<&str, (&str, &str), Error<&str>> {
    separated_pair(
        alpha1,
        tag("-to-"),
        terminated(
            alpha1,
            delimited(space1, tag("map:"), line_ending),
        ),
    )(input)
}

fn parse_map(input: &str) -> IResult<&str, Map> {
    let (input, (title, vecs)) = pair(
        preceded(opt(line_ending), parse_map_title),
        many1(terminated(
            separated_list1(space1, nom_u64),
            line_ending,
        )),
    )(input)?;

    let range_maps = vecs
        .iter()
        .map(|v| {
            let mut it = v.iter();
            let (dst, src, length) = (
                *it.next().unwrap(),
                *it.next().unwrap(),
                *it.next().unwrap(),
            );
            RangeMap {
                src_range: src..src + length,
                dst_range: dst..dst + length,
            }
        })
        .collect();

    Ok((
        input,
        Map {
            src: title.0,
            dst: title.1,
            range_maps,
        },
    ))
}

#[cfg(test)]
mod tests {
    use indoc::indoc;
    use rstest::{fixture, rstest};

    use super::*;

    #[fixture]
    fn input() -> &'static str {
        indoc! {
            r#"
            seeds: 79 14 55 13

            seed-to-soil map:
            50 98 2
            52 50 48

            soil-to-fertilizer map:
            0 15 37
            37 52 2
            39 0 15

            fertilizer-to-water map:
            49 53 8
            0 11 42
            42 0 7
            57 7 4

            water-to-light map:
            88 18 7
            18 25 70

            light-to-temperature map:
            45 77 23
            81 45 19
            68 64 13

            temperature-to-humidity map:
            0 69 1
            1 0 69

            humidity-to-location map:
            60 56 37
            56 93 4
            "#
        }
    }

    #[test]
    fn test_parse_map_title() {
        assert_eq!(
            parse_map_title("humidity-to-location map:\n").unwrap(),
            ("", ("humidity", "location"))
        )
    }

    #[test]
    fn test_parse_map() {
        let (_, output) = parse_map(indoc! {
            r#"
                humidity-to-location map:
                60 56 37
                56 93 4
            "#
        })
        .unwrap();
        // println!("{:?}", output);
        assert_eq!(output.src, "humidity");
        assert_eq!(output.dst, "location");
        assert_eq!(
            vec![
                RangeMap {
                    dst_range: 60..97,
                    src_range: 56..93,
                },
                RangeMap {
                    dst_range: 56..60,
                    src_range: 93..97,
                },
            ],
            output.range_maps
        )
    }

    #[rstest]
    fn test_part1(input: &str) {
        assert_eq!(35, part1(input))
    }

    #[rstest]
    fn test_part2(input: &str) {
        assert_eq!(46, part2(input))
    }
}
