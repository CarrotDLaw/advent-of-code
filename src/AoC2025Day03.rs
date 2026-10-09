fn main() {
  let input = include_str!("input/AoC2025Day03.txt")
    .lines()
    .map(str::trim)
    .filter(|line| !line.is_empty())
    .collect::<Vec<_>>();

  println!("Part 1: {}", part_1(&input));
  println!("Part 2: {}", part_2(&input, 12));
}

fn part_1(input: &[&str]) -> u32 {
  input
    .iter()
    .filter_map(|line| {
      let digits = line
        .chars()
        .filter_map(|x| x.to_digit(10))
        .collect::<Vec<_>>();

      digits[..digits.len().saturating_sub(1)]
        .iter()
        .enumerate()
        .map(|(i, &x)| x * 10 + (digits[(i + 1)..].iter().max().copied().unwrap_or_default()))
        .max()
    })
    .sum()
}

fn part_2(input: &[&str], num_joltage_digit: usize) -> u64 {
  input
    .iter()
    .filter_map(|line| {
      let digits = line
        .chars()
        .filter_map(|x| x.to_digit(10))
        .collect::<Vec<_>>();

      let n = digits.len();
      if n < num_joltage_digit {
        return None;
      }

      let (stack, _) = digits.into_iter().fold(
        (Vec::with_capacity(num_joltage_digit), n - num_joltage_digit),
        |(mut stack, mut drop_budget), digit| {
          while drop_budget > 0 && stack.last().is_some_and(|&top| top < digit) {
            stack.pop();
            drop_budget -= 1;
          }

          if stack.len() < num_joltage_digit {
            stack.push(digit);
          } else {
            drop_budget -= 1;
          }

          (stack, drop_budget)
        },
      );

      Some(
        stack
          .into_iter()
          .fold(0u64, |acc, x| acc * 10 + u64::from(x)),
      )
    })
    .sum()
}

#[cfg(test)]
mod tests {
  use super::*;

  const TEST_INPUT: &str = "987654321111111\n811111111111119\n234234234234278\n818181911112111\n";

  #[test]
  fn test_part_1() {
    let input = TEST_INPUT
      .lines()
      .map(str::trim)
      .filter(|line| !line.is_empty())
      .collect::<Vec<_>>();

    assert_eq!(part_1(&input), (98 + 89 + 78 + 92))
  }

  #[test]
  fn test_part_2() {
    let input = TEST_INPUT
      .lines()
      .map(str::trim)
      .filter(|line| !line.is_empty())
      .collect::<Vec<_>>();

    assert_eq!(part_2(&input, 2), (98 + 89 + 78 + 92));
    assert_eq!(
      part_2(&input, 12),
      (987654321111 + 811111111119 + 434234234278 + 888911112111)
    )
  }
}
