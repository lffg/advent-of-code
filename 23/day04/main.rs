use std::collections::HashSet;

static INPUT: &str = include_str!("input.txt");

fn card_wins(card: &Card) -> usize {
    let winning: HashSet<usize> = card.winning.iter().copied().collect();
    card.have.iter().filter(|n| winning.contains(n)).count()
}

fn part1(input: &str) -> usize {
    fn card_points(card: &Card) -> usize {
        let wins = card_wins(card);
        if 0 < wins {
            2_usize.pow(u32::try_from(wins - 1).unwrap())
        } else {
            0
        }
    }
    let table = parsers::parse_table(input).unwrap();
    table.iter().map(card_points).sum()
}

fn part2(input: &str) -> usize {
    let table = parsers::parse_table(input).unwrap();

    // Map (CardNumber - 1) to the number of copies of the corresponding Card.
    // Initialized with ones since initially we have a single of each card.
    //
    // In this case, since the input is sequential and contiguous, we don't have
    // to use a hash map.
    let mut copies = vec![1; table.len()];

    for card in table {
        let card_index = card.number - 1;
        let wins = card_wins(&card);

        // Only elements that appeared *before* the current card may copy it,
        // so when processing the n-th card, we already know how many copies of
        // it were created.
        let copies_to_process = copies[card_index];

        for _ in 0..copies_to_process {
            let lo = card_index + 1;
            let hi = lo + wins;
            // Registers the copies (of the successive elements) winned by the
            // current card.
            //
            // Notice, due to this outer `for`, that we register the wins for
            // each one of the current card's copies.
            for i in lo..hi {
                if let Some(num) = copies.get_mut(i) {
                    *num += 1;
                }
            }
        }
    }

    copies.into_iter().sum()
}

fn main() {
    println!("part1: {}", part1(INPUT));
    println!("part2: {}", part2(INPUT));
}

#[derive(Debug)]
struct Card {
    #[allow(dead_code)]
    number: usize,
    winning: Vec<usize>,
    have: Vec<usize>,
}

mod parsers {
    use nom::{
        bytes::complete::tag,
        character::complete::{digit1, line_ending, multispace0, space0},
        combinator::{eof, map, map_res, recognize},
        error::Error,
        multi::{many1, separated_list0, separated_list1},
        sequence::{delimited, tuple},
        Err, IResult,
    };

    use super::*;

    pub fn parse_table(input: &str) -> Result<Vec<Card>, Err<Error<&str>>> {
        let (_, (table, _, _)) = tuple((
            //
            separated_list0(line_ending, parse_card),
            multispace0,
            eof,
        ))(input)?;
        Ok(table)
    }

    fn parse_card(input: &str) -> IResult<&str, Card> {
        map(
            tuple((
                tag("Card"),
                space0,
                number,
                tag(":"),
                number_list,
                tag("|"),
                number_list,
            )),
            |(_, _, number, _, winning, _, have)| Card {
                number,
                winning,
                have,
            },
        )(input)
    }

    fn number_list(input: &str) -> IResult<&str, Vec<usize>> {
        delimited(space0, separated_list1(many1(tag(" ")), number), space0)(input)
    }

    fn number(input: &str) -> IResult<&str, usize> {
        map_res(recognize(digit1), str::parse)(input)
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
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"
            ),
            13
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            part2(
                "\
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"
            ),
            30
        );
    }

    #[test]
    fn test_answers() {
        assert_eq!(part1(INPUT), 21105);
        assert_eq!(part2(INPUT), 5329815);
    }
}
