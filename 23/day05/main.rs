use std::collections::HashMap;

use indicatif::{ParallelProgressIterator as _, ProgressIterator as _};
use rayon::iter::{IntoParallelIterator as _, ParallelIterator as _};

static INPUT: &str = include_str!("input.txt");

fn convert<'a>(cs: &'a Conversions<'a>, from: &str, to: &str, value: usize) -> usize {
    if from == to {
        return value;
    }
    let conversion = &cs[from];
    let mapped_value = conversion
        .range_sets
        .iter()
        .find_map(|set| set.convert(value))
        // If there is no conversion, the value is mapped to itself.
        .unwrap_or(value);
    convert(cs, conversion.to, to, mapped_value)
}

fn part1(input: &str) -> usize {
    let almanac = parsers::parse_almanac(input).unwrap();

    almanac
        .seeds
        .into_iter()
        .map(|seed| convert(&almanac.conversions, "seed", "location", seed))
        .min()
        .unwrap()
}

fn part2(input: &str) -> usize {
    fn pairs<T, I>(iter: impl IntoIterator<Item = T, IntoIter = I>) -> impl Iterator<Item = (T, T)>
    where
        I: Iterator<Item = T>,
    {
        let mut iter = iter.into_iter();
        std::iter::from_fn(move || {
            let a = iter.next()?;
            let b = iter.next().expect("elements pairwise");
            Some((a, b))
        })
    }

    let almanac = parsers::parse_almanac(input).unwrap();

    let len = almanac.seeds.len();
    let seeds: Vec<_> = pairs(almanac.seeds)
        .progress_count((len / 2) as u64)
        .flat_map(|(lo, len)| lo..(lo + len))
        .collect();
    seeds
        .into_par_iter()
        .progress()
        .map(|seed| convert(&almanac.conversions, "seed", "location", seed))
        .min()
        .unwrap()
}

fn main() {
    println!("part1: {}", part1(INPUT));
    println!("part2: {}", part2(INPUT));
}

type Range = std::ops::Range<usize>;

#[derive(Debug)]
struct Almanac<'a> {
    seeds: Vec<usize>,
    conversions: Conversions<'a>,
}

type Conversions<'a> = HashMap</* from */ &'a str, Conversion<'a>>;

#[derive(Debug)]
struct Conversion<'a> {
    from: &'a str,
    to: &'a str,
    range_sets: Vec<RangeSet>,
}

#[derive(Debug)]
struct RangeSet {
    src: Range,
    dst: Range,
}

impl RangeSet {
    fn convert(&self, value: usize) -> Option<usize> {
        self.src.contains(&value).then(|| {
            let offset = value - self.src.start;
            self.dst.start + offset
        })
    }
}

mod parsers {
    use nom::{
        bytes::complete::tag,
        character::complete::{alpha1, digit1, multispace0, newline, space1},
        combinator::{eof, map_res, recognize},
        error::Error,
        multi::{count, separated_list1},
        sequence::tuple,
        Err, IResult,
    };

    use super::*;

    // seeds: 79 14 55 13
    //
    // seed-to-soil map:
    // 50 98 2
    // 52 50 48
    //
    // soil-to-fertilizer map:
    // 0 15 37
    // 37 52 2
    // 39 0 15
    pub fn parse_almanac(i: &str) -> Result<Almanac<'_>, Err<Error<&str>>> {
        let sep = |i| count(newline, 2)(i);

        let (i, _) = tag("seeds: ")(i)?;
        let (i, seeds) = number_list(i)?;
        let (i, _) = sep(i)?;
        let (i, conversions) = separated_list1(sep, parse_conversion)(i)?;
        let (i, _) = multispace0(i)?;
        eof(i)?;

        Ok(Almanac {
            seeds,
            conversions: conversions.into_iter().map(|c| (c.from, c)).collect(),
        })
    }

    // seed-to-soil map:
    // 50 98 2
    // 52 50 48
    fn parse_conversion(i: &str) -> IResult<&str, Conversion<'_>> {
        let (i, from) = alpha1(i)?;
        let (i, _) = tag("-to-")(i)?;
        let (i, to) = alpha1(i)?;
        let (i, _) = tag(" map:")(i)?;
        let (i, _) = newline(i)?;
        let (i, range_sets) = separated_list1(newline, parse_range_set)(i)?;

        Ok((
            i,
            Conversion {
                from,
                to,
                range_sets,
            },
        ))
    }

    // 50 98 2
    fn parse_range_set(i: &str) -> IResult<&str, RangeSet> {
        let (i, (dst0, _, src0, _, len)) = tuple((
            //
            number, space1, number, space1, number,
        ))(i)?;
        Ok((
            i,
            RangeSet {
                src: src0..(src0 + len),
                dst: dst0..(dst0 + len),
            },
        ))
    }

    fn number_list(i: &str) -> IResult<&str, Vec<usize>> {
        separated_list1(space1, number)(i)
    }

    fn number(i: &str) -> IResult<&str, usize> {
        map_res(recognize(digit1), str::parse)(i)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(
            part1(
                "\
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
56 93 4"
            ),
            35
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            part2(
                "\
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
56 93 4"
            ),
            46
        );
    }

    #[test]
    fn test_answers() {
        assert_eq!(part1(INPUT), 825516882);
        // assert_eq!(part2(INPUT), 0);
    }
}
