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
