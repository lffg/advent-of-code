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

fn main() {
    static INPUT: &str = include_str!("input.txt");

    println!("part1: {}", part1(INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = include_str!("small-input.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 142);
    }
}
