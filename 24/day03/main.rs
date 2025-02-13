use regex::Regex;

static INPUT: &str = include_str!("input.txt");

fn main() {
    println!("part1: {}", part1(&INPUT));
    println!("part2: {}", part2(&INPUT));
}

fn part1(input: &str) -> i64 {
    let re = Regex::new(r#"mul\((\d{1,3}),(\d{1,3})\)"#).unwrap();

    re.captures_iter(input)
        .map(|c| {
            let get = |i: usize| c[i].parse::<i64>().unwrap();
            get(1) * get(2)
        })
        .sum()
}

fn part2(input: &str) -> i64 {
    let re = Regex::new(r#"do(?:n't)?\(\)|mul\((\d{1,3}),(\d{1,3})\)"#).unwrap();

    let mut enabled = true;
    re.captures_iter(input)
        .filter_map(|c| {
            let m = &c[0];
            match m {
                "do()" | "don't()" => {
                    enabled = m == "do()";
                    return None;
                }
                _ if !enabled => return None,
                _ => (),
            }
            let get = |i: usize| c[i].parse::<i64>().unwrap();
            Some(get(1) * get(2))
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(
            part1(r#"xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"#),
            161
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            part2(r#"xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"#),
            48
        );
    }
}
