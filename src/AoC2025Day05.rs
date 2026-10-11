use std::{cmp::max, ops::RangeInclusive};

trait StringExt {
  fn parse_fresh_id_range(&self) -> Result<RangeInclusive<usize>, ()>;
}

impl StringExt for str {
  fn parse_fresh_id_range(&self) -> Result<RangeInclusive<usize>, ()> {
    let mut parts = self.split('-');
    let start = parts.next().ok_or(())?.parse::<usize>().map_err(|_| ())?;
    let end = parts.next().ok_or(())?.parse::<usize>().map_err(|_| ())?;

    Ok(start..=end)
  }
}

fn main() {
  let input = include_str!("input/AoC2025Day05.txt")
    .lines()
    .map(str::trim)
    .filter(|line| !line.is_empty())
    .collect::<Vec<_>>();
  let fresh_id_ranges = parse_fresh_id_ranges(&input);
  let ids = parse_ids(&input);

  println!("Part 1: {}", part_1(&fresh_id_ranges, &ids));
  println!("Part 2: {}", part_2(&fresh_id_ranges));
}

fn part_1(fresh_id_ranges: &[RangeInclusive<usize>], ids: &[usize]) -> usize {
  ids
    .iter()
    .filter(|&&id| fresh_id_ranges.iter().any(|range| range.contains(&id)))
    .count()
}

fn part_2(fresh_id_ranges: &[RangeInclusive<usize>]) -> usize {
  let mut fresh_id_ranges = fresh_id_ranges.to_vec();
  fresh_id_ranges.sort_unstable_by_key(|range| *range.start());

  let mut fresh_id_ranges_iter = fresh_id_ranges.into_iter().peekable();
  let first_range = fresh_id_ranges_iter.next().unwrap();

  let union = fresh_id_ranges_iter.fold(vec![first_range], |mut acc, range| {
    let last = acc.last_mut().unwrap();

    if *last.end() >= *range.start() - 1 {
      *last = *last.start()..=max(*last.end(), *range.end());
    } else {
      acc.push(range);
    }

    acc
  });

  union.iter().map(|r| r.end() - r.start() + 1).sum()
}

fn parse_fresh_id_ranges(input: &[&str]) -> Vec<RangeInclusive<usize>> {
  input
    .iter()
    .filter_map(|line| line.parse_fresh_id_range().ok())
    .collect()
}

fn parse_ids(input: &[&str]) -> Vec<usize> {
  input
    .iter()
    .filter_map(|line| line.parse::<usize>().ok())
    .collect()
}

#[cfg(test)]
mod tests {
  use super::*;

  const TEST_INPUT: &str = "3-5\n10-14\n16-20\n12-18\n\n1\n5\n8\n11\n17\n32\n";

  #[test]
  fn test_parse_fresh_id_ranges() {
    let input = TEST_INPUT
      .lines()
      .map(str::trim)
      .filter(|line| !line.is_empty())
      .collect::<Vec<_>>();

    assert_eq!(
      parse_fresh_id_ranges(&input),
      vec![3..=5, 10..=14, 16..=20, 12..=18]
    );
  }

  #[test]
  fn test_parse_ids() {
    let input = TEST_INPUT
      .lines()
      .map(str::trim)
      .filter(|line| !line.is_empty())
      .collect::<Vec<_>>();

    assert_eq!(parse_ids(&input), vec![1, 5, 8, 11, 17, 32]);
  }

  #[test]
  fn test_part_1() {
    let input = TEST_INPUT
      .lines()
      .map(str::trim)
      .filter(|line| !line.is_empty())
      .collect::<Vec<_>>();

    let fresh_id_ranges = parse_fresh_id_ranges(&input);
    let ids = parse_ids(&input);

    assert_eq!(part_1(&fresh_id_ranges, &ids), 3);
  }

  #[test]
  fn test_part_2() {
    let input = TEST_INPUT
      .lines()
      .map(str::trim)
      .filter(|line| !line.is_empty())
      .collect::<Vec<_>>();

    let fresh_id_ranges = parse_fresh_id_ranges(&input);

    assert_eq!(part_2(&fresh_id_ranges), 14);
  }
}
