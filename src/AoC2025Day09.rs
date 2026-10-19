use std::{
  cmp::{max, min, Reverse},
  str::FromStr,
};

struct Line {
  start: Point,
  end: Point,
}

impl Line {
  fn new(start: Point, end: Point) -> Self {
    Self { start, end }
  }

  fn min_x(&self) -> isize {
    min(self.start.x, self.end.x)
  }

  fn max_x(&self) -> isize {
    max(self.start.x, self.end.x)
  }

  fn min_y(&self) -> isize {
    min(self.start.y, self.end.y)
  }

  fn max_y(&self) -> isize {
    max(self.start.y, self.end.y)
  }

  fn is_vertical(&self) -> bool {
    self.start.x == self.end.x
  }

  fn is_horizontal(&self) -> bool {
    self.start.y == self.end.y
  }
}

impl Point {
  fn new(x: isize, y: isize) -> Self {
    Self { x, y }
  }

  fn is_inside_polygon(&self, points: &[Point]) -> bool {
    let mut is_inside = false;

    for i in 0..points.len() {
      let p_1 = &points[i];
      let p_2 = &points[(i + 1) % points.len()];

      if ((p_1.y > self.y) != (p_2.y > self.y))
        && (self.x < ((p_2.x - p_1.x) * (self.y - p_1.y) / (p_2.y - p_1.y) + p_1.x))
      {
        is_inside = !is_inside;
      }
    }

    is_inside
  }
}

#[derive(Clone, Copy)]
struct Point {
  x: isize,
  y: isize,
}

impl FromStr for Point {
  type Err = ();

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let parts = s.split(",").collect::<Vec<_>>();

    if parts.len() != 2 {
      return Err(());
    }

    Ok(Self {
      x: parts.first().ok_or(())?.parse::<isize>().map_err(|_| ())?,
      y: parts.get(1).ok_or(())?.parse::<isize>().map_err(|_| ())?,
    })
  }
}

#[derive(Clone, Copy)]
struct Rectangle {
  corner_1: Point,
  corner_2: Point,
}

impl Rectangle {
  fn new(corner_1: Point, corner_2: Point) -> Self {
    Self { corner_1, corner_2 }
  }

  fn area(&self) -> usize {
    let width = self.corner_2.x.abs_diff(self.corner_1.x) + 1;
    let height = self.corner_2.y.abs_diff(self.corner_1.y) + 1;

    width * height
  }

  fn min_x(&self) -> isize {
    min(self.corner_1.x, self.corner_2.x)
  }

  fn max_x(&self) -> isize {
    max(self.corner_1.x, self.corner_2.x)
  }

  fn min_y(&self) -> isize {
    min(self.corner_1.y, self.corner_2.y)
  }

  fn max_y(&self) -> isize {
    max(self.corner_1.y, self.corner_2.y)
  }

  fn is_line_crossing(&self, line: &Line) -> bool {
    if line.is_vertical() {
      if line.min_x() <= self.min_x() || self.max_x() <= line.min_x() {
        return false;
      }

      if max(self.min_y(), line.min_y()) < min(line.max_y(), self.max_y()) {
        return true;
      }
    }

    if line.is_horizontal() {
      if line.min_y() <= self.min_y() || self.max_y() <= line.min_y() {
        return false;
      }

      if max(self.min_x(), line.min_x()) < min(line.max_x(), self.max_x()) {
        return true;
      }
    }

    false
  }

  fn centre(&self) -> Point {
    Point::new(
      (self.min_x() + self.max_x()) / 2,
      (self.min_y() + self.max_y()) / 2,
    )
  }
}

fn main() {
  let input = include_str!("input/AoC2025Day09.txt")
    .lines()
    .map(str::trim)
    .filter_map(|line| (!line.trim().is_empty()).then_some(line.parse::<Point>().ok()?))
    .collect::<Vec<_>>();

  println!("Part 1: {}", part_1(&input));
  println!("Part 2: {}", part_2(&input));
}

fn part_1(points: &[Point]) -> usize {
  let rectangles = (0..points.len())
    .flat_map(|i| ((i + 1)..points.len()).map(move |j| Rectangle::new(points[i], points[j])))
    .collect::<Vec<_>>();

  rectangles
    .iter()
    .map(|rectangle| rectangle.area())
    .max()
    .unwrap()
}

fn part_2(points: &[Point]) -> usize {
  let mut rectangles = (0..points.len())
    .flat_map(|i| ((i + 1)..points.len()).map(move |j| Rectangle::new(points[i], points[j])))
    .collect::<Vec<_>>();
  rectangles.sort_unstable_by_key(|rectangle| Reverse(rectangle.area()));

  let tiles = (0..points.len())
    .map(|i| Line::new(points[i], points[(i + 1) % points.len()]))
    .collect::<Vec<_>>();

  rectangles
    .iter()
    .find(|rectangle| {
      if tiles.iter().any(|tile| rectangle.is_line_crossing(tile)) {
        return false;
      }

      rectangle.centre().is_inside_polygon(points)
    })
    .map(|rectangle| rectangle.area())
    .unwrap()
}

#[cfg(test)]
mod tests {
  use super::*;

  const TEST_INPUT: &str = "7,1\n11,1\n11,7\n9,7\n9,5\n2,5\n2,3\n7,3\n";

  #[test]
  fn test_part_1() {
    let input = TEST_INPUT
      .lines()
      .map(str::trim)
      .filter_map(|line| (!line.trim().is_empty()).then_some(line.parse::<Point>().ok()?))
      .collect::<Vec<_>>();

    assert_eq!(part_1(&input), 50);
  }

  #[test]
  fn test_part_2() {
    let input = TEST_INPUT
      .lines()
      .map(str::trim)
      .filter_map(|line| (!line.trim().is_empty()).then_some(line.parse::<Point>().ok()?))
      .collect::<Vec<_>>();

    assert_eq!(part_2(&input), 24);
  }
}
