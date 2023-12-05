use std::{
    collections::{HashMap, HashSet},
    ops::Index,
    str::FromStr,
};

static INPUT: &str = include_str!("input.txt");

fn part1(input: &str) -> u32 {
    let grid: Grid = input.parse().expect("valid grid");

    let mut sum = 0;

    let mut iter = grid.entries().peekable();
    loop {
        // Stop the iteration when there are no elements left.
        let Some((coord @ (row, _), value)) = iter.next() else {
            break;
        };
        // The rest of the loop accumulates over digits, so skip non digits.
        let Value::Digit(digit) = value else {
            continue;
        };

        // Initializing the digits accumulator to get the "complete" number.
        let mut number = *digit;

        // The accumulated number will be of use only if the number is neighbor
        // of some special value, so we better also keep track of it.
        let mut seen_special = grid.has_special_neighbors(coord);

        loop {
            // If there are no entries next, or if the next entry is not
            // a digit, we stop accumulating.
            let Some((next_coord, Value::Digit(next_digit))) = iter.peek() else {
                break;
            };
            // If the next digit is not in the same row as the previous
            // digits, then they are part of different numbers, so we
            // also stop accumulating.
            if row != next_coord.0 {
                break;
            }

            seen_special |= grid.has_special_neighbors(*next_coord);
            number = number * 10 + next_digit;

            iter.next(); // Don't forget me ;)
        }

        if seen_special {
            sum += number;
        }
    }

    sum
}

// Copied from `part1`; changes are highlighted with comments.
fn part2(input: &str) -> u32 {
    let grid: Grid = input.parse().expect("valid grid");

    // Keep a track of cogs adjacent to numbers. Keys are cog coord.
    let mut map = HashMap::<Coord, (/* count */ usize, /* ratio */ u32)>::new();

    let mut iter = grid.entries().peekable();
    loop {
        let Some((coord @ (row, _), value)) = iter.next() else {
            break;
        };
        let Value::Digit(digit) = value else {
            continue;
        };

        let mut number = *digit;

        // Keep track of all cogs adjacent to the number being accumulated.
        let mut cogs_seen: HashSet<_> = grid.cog_neighbors(coord).collect();

        loop {
            let Some((next_coord, Value::Digit(next_digit))) = iter.peek() else {
                break;
            };
            if row != next_coord.0 {
                break;
            }

            // Add more adjacent cogs, if needed.
            cogs_seen.extend(grid.cog_neighbors(*next_coord));
            number = number * 10 + next_digit;

            iter.next();
        }

        // Increment number count of cogs in the map.
        for cog_coord in cogs_seen {
            let (count, ratio) = map.entry(cog_coord).or_insert((0, 1));
            *count += 1;
            *ratio *= number;
        }
    }

    // Return the sum of cog ratios with 2 elements.
    map.into_values()
        .filter_map(|(count, ratio)| (count == 2).then_some(ratio))
        .sum()
}

fn main() {
    println!("part1: {}", part1(INPUT));
    println!("part2: {}", part2(INPUT));
}

type Coord = (usize, usize);

#[derive(Debug)]
enum Value {
    Digit(u32),
    Special(char),
    Empty,
}

impl TryFrom<char> for Value {
    type Error = String;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        Ok(match value {
            '.' => Value::Empty,
            c if c.is_ascii_digit() => Value::Digit(c.to_digit(10).unwrap()),
            c => Value::Special(c),
        })
    }
}

#[derive(Debug)]
struct Grid {
    grid: Vec<Value>,
    width: usize,
    height: usize,
}

impl Index<Coord> for Grid {
    type Output = Value;

    fn index(&self, (row, col): Coord) -> &Self::Output {
        &self.grid[row * self.width + col]
    }
}

impl FromStr for Grid {
    type Err = String;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let width = input.lines().next().expect("at least one line").len();
        let mut height = 0;

        let grid = input
            .lines()
            .flat_map(|line| {
                height += 1;
                line.chars()
            })
            .map(|c| Value::try_from(c).expect("valid char to value"))
            .collect();

        Ok(Grid {
            grid,
            width,
            height,
        })
    }
}

impl Grid {
    fn entries(&self) -> impl Iterator<Item = (Coord, &Value)> {
        let w = self.width;
        self.grid
            .iter()
            .enumerate()
            .map(move |(i, v)| ((i / w, i % w), v))
    }

    fn neighbors(&self, (row, col): Coord) -> impl Iterator<Item = Coord> + '_ {
        static OFFSETS: [(isize, isize); 8] = [
            (-1, -1), // top left
            (-1, 0),  // top center
            (-1, 1),  // top right
            (0, -1),  // center left
            (0, 1),   // center right
            (1, -1),  // bottom left
            (1, 0),   // bottom center
            (1, 1),   // bottom right
        ];
        fn bounded_add(bound: usize, a: usize, b: isize) -> Option<usize> {
            let r = a.checked_add_signed(b)?;
            (r < bound).then_some(r)
        }
        OFFSETS
            .into_iter()
            .filter_map(move |(row_offset, col_offset)| {
                Some((
                    bounded_add(self.height, row, row_offset)?,
                    bounded_add(self.width, col, col_offset)?,
                ))
            })
    }

    fn special_neighbors(&self, coord: Coord) -> impl Iterator<Item = (Coord, char)> + '_ {
        self.neighbors(coord).filter_map(|coord| {
            if let Value::Special(char) = self[coord] {
                Some((coord, char))
            } else {
                None
            }
        })
    }

    fn has_special_neighbors(&self, coord: Coord) -> bool {
        self.special_neighbors(coord).next().is_some()
    }

    fn cog_neighbors(&self, coord: Coord) -> impl Iterator<Item = Coord> + '_ {
        self.special_neighbors(coord)
            .filter_map(|(coord, char)| (char == '*').then_some(coord))
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
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."
            ),
            4361
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            part2(
                "\
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."
            ),
            467835
        );
    }

    #[test]
    fn test_answers() {
        assert_eq!(part1(INPUT), 546563);
        assert_eq!(part2(INPUT), 91031374);
    }
}
