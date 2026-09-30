fn main() {
  let input = include_str!("input/AoC2024Day02.txt")
    .trim()
    .lines()
    .map(|line| {
      line
        .split_whitespace()
        .filter_map(|s| s.parse::<usize>().ok())
        .collect::<Vec<_>>()
    })
    .collect::<Vec<_>>();

  let part_1 = input.iter().filter(|r| is_safe(r)).count();
  let part_2 = input.iter().filter(|r| is_safe_tolerated(r)).count();

  println!("Part 1: {part_1}");
  println!("Part 2: {part_2}");
}

fn is_safe(arr: &[usize]) -> bool {
  (is_increasing(arr) || is_decreasing(arr)) && is_valid_adjacent_difference(arr)
}

fn is_safe_tolerated(arr: &[usize]) -> bool {
  if is_safe(arr) {
    return true;
  }

  (0..arr.len()).any(|i| {
    let arr_tolerated = arr
      .iter()
      .enumerate()
      .filter_map(|(j, x)| (i != j).then_some(*x))
      .collect::<Vec<_>>();

    is_safe(&arr_tolerated)
  })
}

fn is_increasing(arr: &[usize]) -> bool {
  arr.windows(2).all(|w| w[0] < w[1])
}

fn is_decreasing(arr: &[usize]) -> bool {
  arr.windows(2).all(|w| w[0] > w[1])
}

fn is_valid_adjacent_difference(arr: &[usize]) -> bool {
  arr
    .windows(2)
    .all(|w| (1..=3).contains(&(w[0].abs_diff(w[1]))))
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_is_safe() {
    assert!(is_safe(&[7, 6, 4, 2, 1]));
    assert!(!is_safe(&[1, 2, 7, 8, 9]));
    assert!(!is_safe(&[9, 7, 6, 2, 1]));
    assert!(!is_safe(&[1, 3, 2, 4, 5]));
    assert!(!is_safe(&[8, 6, 4, 4, 1]));
    assert!(is_safe(&[1, 3, 6, 7, 9]));
  }

  #[test]
  fn test_is_safe_tolerated() {
    assert!(is_safe_tolerated(&[7, 6, 4, 2, 1]));
    assert!(!is_safe_tolerated(&[1, 2, 7, 8, 9]));
    assert!(!is_safe_tolerated(&[9, 7, 6, 2, 1]));
    assert!(is_safe_tolerated(&[1, 3, 2, 4, 5]));
    assert!(is_safe_tolerated(&[8, 6, 4, 4, 1]));
    assert!(is_safe_tolerated(&[1, 3, 6, 7, 9]));
  }
}
