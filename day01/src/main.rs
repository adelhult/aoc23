use std::fs;

fn part1(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let mut digits = line.chars().filter_map(|c| c.to_digit(10));
            let first = digits.next().unwrap();
            let last = digits.last().unwrap_or(first);
            first * 10 + last
        })
        .sum()
}

fn part2(input: &str) -> u32 {
    let numbers = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ]
    .into_iter()
    .enumerate()
    .map(|(d, s)| (d + 1, s));

    input
        .lines()
        .map(|line| {
            let arabic_digits = line
                .chars()
                .enumerate()
                .filter_map(|(i, c)| c.to_digit(10).map(|d| (i, d)));

            let (_, first) = {
                let first_alpha = numbers
                    .clone()
                    .filter_map(|(d, s)| line.find(s).map(|i| (i, d as u32)))
                    .min_by_key(|(i, _)| *i);

                [arabic_digits.clone().next(), first_alpha]
                    .into_iter()
                    .flatten()
                    .min_by_key(|(i, _)| *i)
            }
            .unwrap();

            let (_, last) = {
                let last_alpha = numbers
                    .clone()
                    .filter_map(|(d, s)| line.rfind(s).map(|i| (i, d as u32)))
                    .max_by_key(|(i, _)| *i);

                [arabic_digits.last(), last_alpha]
                    .into_iter()
                    .flatten()
                    .max_by_key(|(i, _)| *i)
            }
            .unwrap();

            first * 10 + last
        })
        .sum()
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    println!("{}", part1(&input));
    println!("{}", part2(&input));
}
