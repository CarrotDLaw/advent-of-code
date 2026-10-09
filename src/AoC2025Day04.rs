use std::collections::HashSet;

type Point = (isize, isize);

trait PointExt {
  fn neighbours(&self) -> impl Iterator<Item = Point> + '_;
}

impl PointExt for Point {
  fn neighbours(&self) -> impl Iterator<Item = Point> + '_ {
    DIRS.iter().map(move |(dx, dy)| (self.0 + dx, self.1 + dy))
  }
}

const DIRS: [(isize, isize); 8] = [
  (-1, -1),
  (-1, 0),
  (-1, 1),
  (0, -1),
  (0, 1),
  (1, -1),
  (1, 0),
  (1, 1),
];

fn main() {
  let input = include_str!("input/AoC2025Day04.txt")
    .lines()
    .map(str::trim)
    .filter(|line| !line.is_empty())
    .collect::<Vec<_>>();
  let rolls = parse(&input);

  println!("Part 1: {}", part_1(&rolls));
  println!("Part 2: {}", part_2(rolls));
}

fn part_1(rolls: &HashSet<Point>) -> usize {
  rolls.iter().filter(|&&p| is_accessible(p, rolls)).count()
}

fn part_2(mut rolls: HashSet<Point>) -> usize {
  let mut count = 0;

  loop {
    let accessible_points = rolls
      .iter()
      .filter(|&&p| is_accessible(p, &rolls))
      .copied()
      .collect::<Vec<_>>();

    if accessible_points.is_empty() {
      break;
    }

    count += accessible_points.len();
    for point in accessible_points {
      rolls.remove(&point);
    }
  }

  count
}

fn parse(input: &[&str]) -> HashSet<Point> {
  input
    .iter()
    .enumerate()
    .flat_map(|(i, line)| {
      line
        .chars()
        .enumerate()
        .filter_map(move |(j, c)| (c == '@').then_some((i.try_into().ok()?, j.try_into().ok()?)))
    })
    .collect()
}

fn is_accessible(point: Point, rolls: &HashSet<Point>) -> bool {
  (0..4).contains(&point.neighbours().filter(|n| rolls.contains(n)).count())
}

#[cfg(test)]
mod tests {
  use super::*;

  const TEST_INPUT: &str = "..@@.@@@@.\n@@@.@.@.@@\n@@@@@.@.@@\n@.@@@@..@.\n@@.@@@@.@@\n.@@@@@@@.@\n.@.@.@.@@@\n@.@@@.@@@@\n.@@@@@@@@.\n@.@.@@@.@.\n";

  #[test]
  fn test_part_1() {
    let input = TEST_INPUT
      .lines()
      .map(str::trim)
      .filter(|line| !line.is_empty())
      .collect::<Vec<_>>();

    assert_eq!(part_1(&parse(&input)), 13);
  }

  #[test]
  fn test_part_2() {
    let input = TEST_INPUT
      .lines()
      .map(str::trim)
      .filter(|line| !line.is_empty())
      .collect::<Vec<_>>();

    assert_eq!(part_2(parse(&input)), 43);
  }
}
