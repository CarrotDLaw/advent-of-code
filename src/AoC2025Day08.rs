use std::{cmp::Reverse, str::FromStr};

struct Point {
  x: usize,
  y: usize,
  z: usize,
}

impl FromStr for Point {
  type Err = ();

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let parts = s.split(",").collect::<Vec<_>>();

    if parts.len() != 3 {
      return Err(());
    }

    Ok(Self {
      x: parts.first().ok_or(())?.parse::<usize>().map_err(|_| ())?,
      y: parts.get(1).ok_or(())?.parse::<usize>().map_err(|_| ())?,
      z: parts.get(2).ok_or(())?.parse::<usize>().map_err(|_| ())?,
    })
  }
}

impl Point {
  fn euclidean_distance(&self, other: &Self) -> usize {
    let dx = self.x.abs_diff(other.x);
    let dy = self.y.abs_diff(other.y);
    let dz = self.z.abs_diff(other.z);

    dx * dx + dy * dy + dz * dz
  }
}

struct DisjointSet {
  parents: Vec<usize>,
  sizes: Vec<usize>,
  circuit_count: usize,
}

impl DisjointSet {
  fn new(n: usize) -> Self {
    Self {
      parents: (0..n).collect(),
      sizes: vec![1; n],
      circuit_count: n,
    }
  }

  fn find_root_and_compress(&mut self, x: usize) -> usize {
    if self.parents[x] != x {
      self.parents[x] = self.find_root_and_compress(self.parents[x]);
    }

    self.parents[x]
  }

  fn try_union(&mut self, a: usize, b: usize) -> bool {
    let root_a = self.find_root_and_compress(a);
    let root_b = self.find_root_and_compress(b);

    if root_a == root_b {
      return false;
    }

    if self.sizes[root_a] < self.sizes[root_b] {
      self.parents[root_a] = root_b;
      self.sizes[root_b] += self.sizes[root_a];
    } else {
      self.parents[root_b] = root_a;
      self.sizes[root_a] += self.sizes[root_b];
    }

    self.update_circuit_count();
    true
  }

  fn sorted_sizes(&mut self) -> Vec<usize> {
    let mut sizes = Vec::new();
    for i in 0..self.parents.len() {
      if self.find_root_and_compress(i) == i {
        sizes.push(self.sizes[i]);
      }
    }

    sizes.sort_unstable_by_key(|&x| Reverse(x));
    sizes
  }

  fn update_circuit_count(&mut self) {
    self.circuit_count -= 1;
  }

  fn is_connected(&self) -> bool {
    self.circuit_count == 1
  }
}

fn main() {
  let input = include_str!("input/AoC2025Day08.txt")
    .lines()
    .map(str::trim)
    .filter_map(|line| (!line.is_empty()).then_some(line.parse::<Point>().ok()?))
    .collect::<Vec<_>>();

  println!("Part 1: {}", part_1(&input, None, None));
  println!("Part 2: {}", part_2(&input));
}

fn part_1(points: &[Point], j: Option<usize>, c: Option<usize>) -> usize {
  let j = j.unwrap_or(1000);
  let c = c.unwrap_or(3);
  let n = points.len();

  let mut pairs = Vec::new();
  for i in 0..n {
    for j in (i + 1)..n {
      pairs.push((i, j, points[i].euclidean_distance(&points[j])));
    }
  }

  pairs.sort_unstable_by_key(|&(_, _, dist)| dist);

  let mut disjoint_set = DisjointSet::new(n);
  for &(u, v, _) in pairs.iter().take(j) {
    disjoint_set.try_union(u, v);
  }

  disjoint_set.sorted_sizes().iter().take(c).product()
}

fn part_2(points: &[Point]) -> usize {
  let n = points.len();

  let mut pairs = Vec::new();
  for i in 0..n {
    for j in (i + 1)..n {
      pairs.push((i, j, points[i].euclidean_distance(&points[j])));
    }
  }

  pairs.sort_unstable_by_key(|&(_, _, dist)| dist);

  let mut disjoint_set = DisjointSet::new(n);
  pairs
    .iter()
    .filter_map(|&(u, v, _)| {
      if !disjoint_set.try_union(u, v) {
        return None;
      }

      if !disjoint_set.is_connected() {
        return None;
      }

      Some(points[u].x * points[v].x)
    })
    .next()
    .unwrap()
}

#[cfg(test)]
mod tests {
  use super::*;

  const TEST_INPUT: &str = "162,817,812\n57,618,57\n906,360,560\n592,479,940\n352,342,300\n466,668,158\n542,29,236\n431,825,988\n739,650,466\n52,470,668\n216,146,977\n819,987,18\n117,168,530\n805,96,715\n346,949,466\n970,615,88\n941,993,340\n862,61,35\n984,92,344\n425,690,689\n";

  #[test]
  fn test_part_1() {
    let input = TEST_INPUT
      .lines()
      .filter_map(|line| line.parse::<Point>().ok())
      .collect::<Vec<_>>();

    assert_eq!(part_1(&input, Some(10), None), (5 * 4 * 2));
  }

  #[test]
  fn test_part_2() {
    let input = TEST_INPUT
      .lines()
      .filter_map(|line| line.parse::<Point>().ok())
      .collect::<Vec<_>>();

    assert_eq!(part_2(&input), (216 * 117));
  }
}
