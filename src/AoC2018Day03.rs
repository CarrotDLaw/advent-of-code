use std::str::FromStr;

const FABRIC_LENGTH: usize = 1000;

struct Claim {
  id: usize,
  x: usize,
  y: usize,
  width: usize,
  height: usize,
}

impl FromStr for Claim {
  type Err = ();

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    // e.g., "#123 @ 3,2: 5x4"
    let parts = s
      .split(|c| "#@, :x".contains(c))
      .filter(|p| !p.is_empty())
      .collect::<Vec<_>>();

    if parts.len() != 5 {
      return Err(());
    }

    Ok(Claim {
      id: parts.first().and_then(|p| p.parse().ok()).ok_or(())?,
      x: parts.get(1).and_then(|p| p.parse().ok()).ok_or(())?,
      y: parts.get(2).and_then(|p| p.parse().ok()).ok_or(())?,
      width: parts.get(3).and_then(|p| p.parse().ok()).ok_or(())?,
      height: parts.get(4).and_then(|p| p.parse().ok()).ok_or(())?,
    })
  }
}

fn main() {
  let mut fabric = vec![vec![0; FABRIC_LENGTH]; FABRIC_LENGTH];
  let claims = include_str!("input/AoC2018Day03.txt")
    .trim()
    .lines()
    .filter_map(|line| line.parse::<Claim>().ok())
    .collect::<Vec<_>>();

  for claim in &claims {
    for row in fabric.iter_mut().skip(claim.x).take(claim.width) {
      for cell in row.iter_mut().skip(claim.y).take(claim.height) {
        *cell += 1;
      }
    }
  }

  println!(
    "Part 1: {}",
    fabric
      .iter()
      .flat_map(|row| row.iter())
      .filter(|&&cell| cell > 1)
      .count()
  );

  let non_overlapping_claim = claims
    .iter()
    .find(|claim| {
      fabric.iter().skip(claim.x).take(claim.width).all(|row| {
        row
          .iter()
          .skip(claim.y)
          .take(claim.height)
          .all(|&cell| cell == 1)
      })
    })
    .map(|claim| claim.id);

  println!("Part 2: {}", non_overlapping_claim.unwrap_or(0));
}
