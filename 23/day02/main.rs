use std::{collections::HashMap, str::FromStr};

fn part1(input: &str) -> u32 {
    fn valid_set(set: &Set) -> bool {
        set.colors
            .iter()
            .all(|(color, used)| used <= &color.maximum_allowed())
    }

    fn valid_game(game: &Game) -> bool {
        game.sets.iter().all(valid_set)
    }

    input
        .lines()
        .map(|line| line.parse::<Game>().expect("syntactically correct game"))
        .filter(valid_game)
        .map(|game| game.id)
        .sum()
}

fn part2(input: &str) -> u32 {
    fn game_power(game: &Game) -> u32 {
        game.sets
            .iter()
            // Create a hashmap with the biggest required value for each color.
            .fold(HashMap::new(), |mut map, set| {
                for (color, &used) in &set.colors {
                    let val = map.entry(*color).or_default();
                    if *val < used {
                        *val = used;
                    }
                }
                map
            })
            .values()
            .product()
    }

    input
        .lines()
        .map(|line| line.parse::<Game>().expect("syntactically correct game"))
        .map(|game| game_power(&game))
        .sum()
}

fn main() {
    static INPUT: &str = include_str!("input.txt");

    println!("part1: {}", part1(INPUT));
    println!("part2: {}", part2(INPUT));
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
enum Color {
    Red,
    Green,
    Blue,
}

impl Color {
    pub fn maximum_allowed(&self) -> u32 {
        match self {
            Color::Red => 12,
            Color::Green => 13,
            Color::Blue => 14,
        }
    }
}

#[derive(Debug)]
struct Set {
    colors: HashMap<Color, u32>,
}

#[derive(Debug)]
struct Game {
    id: u32,
    sets: Vec<Set>,
}

impl FromStr for Game {
    type Err = String; // XX: Maybe create a better error type.

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parsers::game(s)
            .map(|(_, game)| game)
            .map_err(|e| e.to_string())
    }
}

mod parsers {
    use nom::{
        branch::alt,
        bytes::complete::tag,
        character::complete::digit1,
        combinator::{map, map_res, opt, recognize},
        multi::separated_list1,
        sequence::tuple,
        IResult,
    };

    use super::*;

    // Game 51: 2 green, 6 blue; 1 green, 10 blue, 1 red; 3 blue, 2 green
    pub(crate) fn game(input: &str) -> IResult<&str, Game> {
        let (input, (_, _, id, _, _, sets)) = tuple((
            //
            tag("Game"),
            space_opt,
            number,
            tag(":"),
            space_opt,
            sets,
        ))(input)?;
        Ok((input, Game { id, sets }))
    }

    // 2 green, 6 blue; 1 green, 10 blue, 1 red; 3 blue, 2 green
    fn sets(input: &str) -> IResult<&str, Vec<Set>> {
        separated_list1(tuple((tag(";"), space_opt)), set)(input)
    }

    // 2 green, 6 blue
    fn set(input: &str) -> IResult<&str, Set> {
        let (input, entries) = separated_list1(tuple((tag(","), space_opt)), color_count)(input)?;
        let colors = entries.into_iter().collect();
        Ok((input, Set { colors }))
    }

    // 2 green
    fn color_count(input: &str) -> IResult<&str, (Color, u32)> {
        let (input, (count, _, color)) = tuple((number, space_opt, color))(input)?;
        Ok((input, (color, count)))
    }

    // green
    fn color(input: &str) -> IResult<&str, Color> {
        alt((
            map(tag("red"), |_| Color::Red),
            map(tag("green"), |_| Color::Green),
            map(tag("blue"), |_| Color::Blue),
        ))(input)
    }

    fn space_opt(input: &str) -> IResult<&str, Option<&str>> {
        opt(tag(" "))(input)
    }

    fn number(input: &str) -> IResult<&str, u32> {
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
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"
            ),
            8
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            part2(
                "\
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"
            ),
            2286
        );
    }
}
