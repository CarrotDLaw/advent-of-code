trait CharExt {
  fn is_air(&self) -> bool;
  fn is_splitter(&self) -> bool;
  fn is_tachyon(&self) -> bool;
}

impl CharExt for char {
  fn is_air(&self) -> bool {
    matches!(self, '.')
  }

  fn is_splitter(&self) -> bool {
    matches!(self, '^')
  }

  fn is_tachyon(&self) -> bool {
    matches!(self, '|')
  }
}

fn main() {
  let input = include_str!("input/AoC2025Day07.txt")
    .lines()
    .map(str::trim)
    .filter(|line| !line.is_empty())
    .collect::<Vec<_>>();

  println!("Part 1: {}", part_1(&input));
  println!("Part 2: {}", part_2(&input));
}

fn part_1(input: &[&str]) -> usize {
  let num_col = input[0].len();
  let s_idx = input[0].find("S").unwrap();
  let mut input = input
    .iter()
    .map(|line| line.chars().collect::<Vec<_>>())
    .collect::<Vec<_>>();
  input[0][s_idx] = '|';

  let mut count = 0;
  for y in 0..input.len() - 1 {
    for x in 0..num_col {
      if input[y][x].is_air() || input[y][x].is_splitter() {
        continue;
      }

      if input[y][x].is_tachyon() && input[y + 1][x].is_air() {
        input[y + 1][x] = '|';
      }

      if input[y][x].is_tachyon() && input[y + 1][x].is_splitter() {
        if x > 0 {
          input[y + 1][x - 1] = '|';
        }

        if x < num_col - 1 {
          input[y + 1][x + 1] = '|';
        }

        count += 1;
      }
    }
  }

  count
}

fn part_2(input: &[&str]) -> usize {
  let num_col = input[0].len();
  let s_idx = input[0].find("S").unwrap();
  let input = input
    .iter()
    .map(|line| line.chars().collect::<Vec<_>>())
    .collect::<Vec<_>>();

  let mut ways = vec![0; num_col];
  ways[s_idx] = 1;

  for y in 0..input.len() - 1 {
    let mut next_ways = vec![0; num_col];
    for x in 0..num_col {
      if ways[x] == 0 {
        continue;
      }

      if !input[y][x].is_splitter() {
        next_ways[x] += ways[x];
        continue;
      }

      if x > 0 {
        next_ways[x - 1] += ways[x];
      }

      if x < num_col - 1 {
        next_ways[x + 1] += ways[x];
      }
    }

    ways = next_ways;
  }

  ways.iter().sum()
}

#[cfg(test)]
mod tests {
  use super::*;

  const TEST_INPUT: &str = ".......S.......\n...............\n.......^.......\n...............\n......^.^......\n...............\n.....^.^.^.....\n...............\n....^.^...^....\n...............\n...^.^...^.^...\n...............\n..^...^.....^..\n...............\n.^.^.^.^.^...^.\n...............\n";

  #[test]
  fn test_part_1() {
    let input = TEST_INPUT
      .lines()
      .map(str::trim)
      .filter(|line| !line.is_empty())
      .collect::<Vec<_>>();

    assert_eq!(part_1(&input), 21);
  }

  #[test]
  fn test_part_2() {
    let input = TEST_INPUT
      .lines()
      .map(str::trim)
      .filter(|line| !line.is_empty())
      .collect::<Vec<_>>();

    assert_eq!(part_2(&input), 40);
  }
}
