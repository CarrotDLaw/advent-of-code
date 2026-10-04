fn main() {
  let input = include_str!("input/AoC2024Day03.txt");

  println!("Part 1: {}", part_1(input));
  println!("Part 2: {}", part_2(input));
}

fn part_1(input: &str) -> usize {
  input
    .match_indices("mul(")
    .filter_map(|(i, _)| parse_mul(&input[i..]))
    .map(|(x, y)| x * y)
    .sum()
}

fn part_2(input: &str) -> usize {
  const DON_T: &str = "don't()";
  const DO: &str = "do()";

  let dont_idx = input.find(DON_T);

  part_1(&input[..dont_idx.unwrap_or(input.len())])
    + input[dont_idx.unwrap_or_default()..]
      .split(DON_T)
      .filter_map(|s| s.find(DO).map(|idx| part_1(&s[(idx + DO.len())..])))
      .sum::<usize>()
}

fn parse_mul(input: &str) -> Option<(usize, usize)> {
  const MUL: &str = "mul(";
  const COMMA: &str = ",";
  const RIGHT_BRACKET: &str = ")";

  let s = input.strip_prefix(MUL)?;
  let comma_idx = s.find(COMMA)?;
  let x_str = &s[..comma_idx];

  if x_str.trim().is_empty() || x_str.parse::<usize>().is_err() {
    return None;
  }

  let s = &s[(comma_idx + 1)..];
  let right_bracket_idx = s.find(RIGHT_BRACKET)?;
  let y_str = &s[..right_bracket_idx];

  if y_str.trim().is_empty() || y_str.parse::<usize>().is_err() {
    return None;
  }

  let x_num = x_str.trim().parse::<usize>().ok()?;
  let y_num = y_str.trim().parse::<usize>().ok()?;

  Some((x_num, y_num))
}
