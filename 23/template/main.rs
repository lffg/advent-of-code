static INPUT: &str = include_str!("input.txt");

fn part1(_input: &str) -> u32 {
    0
}

fn part2(_input: &str) -> u32 {
    0
}

fn main() {
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
                "\
Foo
Bar
Baz"
            ),
            000
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            part2(
                "\
Foo
Bar
Baz"
            ),
            000
        );
    }

    #[test]
    fn test_answers() {
        assert_eq!(part1(INPUT), 0);
        assert_eq!(part2(INPUT), 0);
    }
}
