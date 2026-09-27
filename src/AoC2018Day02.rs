use std::collections::HashMap;

fn main() {
    let input = include_str!("input/AoC2018Day02.txt")
        .trim()
        .lines()
        .collect::<Vec<_>>();

    println!("Part 1: {}", part_1(&input));
    println!("Part 2: {}", part_2(&input));
}

fn part_1(input: &[&str]) -> usize {
    let mut count_two = 0;
    let mut count_three = 0;

    for s in input {
        let mut counts_map = HashMap::<char, u8>::new();
        for c in s.chars() {
            counts_map.entry(c).and_modify(|e| *e += 1).or_insert(1);
        }

        if counts_map.values().any(|&v| v == 2) {
            count_two += 1;
        }

        if counts_map.values().any(|&v| v == 3) {
            count_three += 1;
        }
    }

    count_two * count_three
}

fn part_2(input: &[&str]) -> String {
    for i in 0..input.len() {
        for j in (i + 1)..input.len() {
            let string_i = input[i];
            let string_j = input[j];

            let common_chars = string_i
                .chars()
                .zip(string_j.chars())
                .filter_map(|(a, b)| (a == b).then_some(a))
                .collect::<String>();

            if string_i.len() - common_chars.len() == 1 {
                return common_chars;
            }
        }
    }

    String::new()
}
