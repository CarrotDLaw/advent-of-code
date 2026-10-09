use std::collections::HashMap;

trait CharExt {
  fn is_valid_symbol(&self) -> bool;
  fn is_asterisk(&self) -> bool;
}

impl CharExt for char {
  fn is_valid_symbol(&self) -> bool {
    !self.is_ascii_digit() && *self != '.'
  }

  fn is_asterisk(&self) -> bool {
    *self == '*'
  }
}

struct PartNumber {
  value: usize,
  row: usize,
  col_start: usize,
  col_end: usize,
}

impl PartNumber {
  fn new(value: usize, row: usize, col_start: usize, col_end: usize) -> Self {
    Self {
      value,
      row,
      col_start,
      col_end,
    }
  }

  fn is_adjacent_to(&self, r: usize, c: usize) -> bool {
    let row_start = self.row.saturating_sub(1);
    let row_end = self.row + 1;
    let col_start = self.col_start.saturating_sub(1);
    let col_end = self.col_end + 1;

    (row_start..=row_end).contains(&r) && (col_start..=col_end).contains(&c)
  }
}

fn main() {
  let input = include_str!("input/AoC2023Day03.txt")
    .trim()
    .lines()
    .map(|line| line.chars().collect::<Vec<_>>())
    .collect::<Vec<_>>();

  println!("Part 1: {}", part_1(&input));
  println!("Part 2: {}", part_2(&input));
}

fn part_1(grid: &[Vec<char>]) -> usize {
  let mut part_numbers = Vec::new();
  for r in 0..grid.len() {
    let mut c = 0;
    while c < grid[r].len() {
      if !grid[r][c].is_ascii_digit() {
        c += 1;
        continue;
      }

      let col_start = c;
      let mut value = 0;
      while c < grid[r].len() && grid[r][c].is_ascii_digit() {
        value = value * 10 + (grid[r][c].to_digit(10).unwrap_or(0) as usize);
        c += 1;
      }

      part_numbers.push(PartNumber::new(value, r, col_start, c - 1))
    }
  }

  part_numbers
    .iter()
    .filter(|n| {
      let row_start = n.row.saturating_sub(1);
      let row_end = (n.row + 1).min(grid.len() - 1);
      let col_start = n.col_start.saturating_sub(1);
      let col_end = (n.col_end + 1).min(grid[n.row].len() - 1);

      grid[row_start..=row_end]
        .iter()
        .flat_map(|row| row[col_start..=col_end].iter())
        .any(|&c| c.is_valid_symbol())
    })
    .map(|n| n.value)
    .sum()
}

fn part_2(grid: &[Vec<char>]) -> usize {
  let symbols = grid
    .iter()
    .enumerate()
    .flat_map(|(r, row)| {
      row
        .iter()
        .enumerate()
        .filter(|(_, &c)| c.is_valid_symbol())
        .map(move |(c, &s)| (r, c, s))
    })
    .collect::<Vec<_>>();

  let mut part_numbers = Vec::new();
  for r in 0..grid.len() {
    let mut c = 0;
    while c < grid[r].len() {
      if !grid[r][c].is_ascii_digit() {
        c += 1;
        continue;
      }

      let col_start = c;
      let mut value = 0;
      while c < grid[r].len() && grid[r][c].is_ascii_digit() {
        value = value * 10 + (grid[r][c].to_digit(10).unwrap_or(0) as usize);
        c += 1;
      }

      part_numbers.push(PartNumber::new(value, r, col_start, c - 1))
    }
  }

  let mut gear_neighbours = HashMap::<(usize, usize), Vec<usize>>::new();
  for &(r, c, s) in &symbols {
    if !s.is_asterisk() {
      continue;
    }

    for n in &part_numbers {
      if n.is_adjacent_to(r, c) {
        gear_neighbours.entry((r, c)).or_default().push(n.value);
      }
    }
  }

  gear_neighbours
    .values()
    .filter(|arr| arr.len() == 2)
    .map(|arr| arr.iter().product::<usize>())
    .sum()
}

#[cfg(test)]
mod tests {
  use super::*;

  const TEST_INPUT: &str = "467..114..\n...*......\n..35..633.\n......#...\n617*......\n.....+.58.\n..592.....\n......755.\n...$.*....\n.664.598..\n";

  #[test]
  fn test_part_1() {
    let input = TEST_INPUT
      .trim()
      .lines()
      .map(|line| line.chars().collect::<Vec<_>>())
      .collect::<Vec<_>>();

    assert_eq!(part_1(&input), 4361);
  }

  #[test]
  fn test_part_2() {
    let input = TEST_INPUT
      .trim()
      .lines()
      .map(|line| line.chars().collect::<Vec<_>>())
      .collect::<Vec<_>>();

    assert_eq!(part_2(&input), 467835);
  }
}
