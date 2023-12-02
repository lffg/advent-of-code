fn part1(input: &str) -> u32 {
    fn parse_line(line: &str) -> u32 {
        let mut iter = line.chars().filter_map(|c| c.to_digit(10));
        let a = iter.next().expect("one digit");
        // If there is a single digit in the line, it won't be encountered
        // again since the iterator already passed over it.
        let b = iter.rev().next().unwrap_or(a);
        a * 10 + b
    }

    input.lines().map(parse_line).sum()
}

fn part2(input: &str) -> u32 {
    static SPELLED_DIGITS: &[&str] = &[
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    fn try_get_digit(s: &str) -> Option<u32> {
        if let digit @ '0'..='9' = s.chars().next()? {
            return Some(digit.to_digit(10).unwrap());
        }
        for (i, sd) in SPELLED_DIGITS.into_iter().enumerate() {
            if s.starts_with(sd) {
                return Some(i as u32 + 1);
            }
        }
        None
    }

    fn parse_line(line: &str) -> u32 {
        let mut iter = line.chars();
        let mut fst = None;
        let mut snd = None;

        while !iter.as_str().is_empty() {
            let s = iter.as_str();
            if let Some(digit) = try_get_digit(s) {
                fst.get_or_insert(digit);
                snd = Some(digit);
            }
            iter.next();
        }

        fst.unwrap() * 10 + snd.unwrap()
    }

    input.lines().map(parse_line).sum()
}

fn main() {
    static INPUT: &str = include_str!("input.txt");

    println!("part1: {}", part1(INPUT));
    println!("part2: {}", part2(INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(
            part1(
                r#"1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet"#
            ),
            142
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            part2(
                r#"two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen"#
            ),
            281
        );
    }
}
