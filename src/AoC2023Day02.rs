struct GameRecord {
  id: usize,
  max_red: usize,
  max_green: usize,
  max_blue: usize,
}

impl GameRecord {
  fn new(id: usize, max_red: usize, max_green: usize, max_blue: usize) -> Self {
    Self {
      id,
      max_red,
      max_green,
      max_blue,
    }
  }

  fn update_max_red(&mut self, count: usize) {
    if count > self.max_red {
      self.max_red = count;
    }
  }

  fn update_max_green(&mut self, count: usize) {
    if count > self.max_green {
      self.max_green = count;
    }
  }

  fn update_max_blue(&mut self, count: usize) {
    if count > self.max_blue {
      self.max_blue = count;
    }
  }
}

fn main() {
  let part_1 = include_str!("input/AoC2023Day02.txt")
    .trim()
    .lines()
    .filter_map(|line| {
      let (game, sets) = line.split_once(":")?;
      let game_id = game
        .trim_start_matches("Game")
        .trim()
        .parse::<usize>()
        .ok()?;

      sets
        .split(";")
        .all(|set| {
          set.split(",").all(|item| {
            let mut item_splitted = item.split_whitespace();
            let count = item_splitted
              .next()
              .and_then(|s| s.parse().ok())
              .unwrap_or(0);

            match item_splitted.next() {
              Some("red") => count <= 12,
              Some("green") => count <= 13,
              Some("blue") => count <= 14,
              _ => false,
            }
          })
        })
        .then_some(game_id)
    })
    .sum::<usize>();

  let part_2 = include_str!("input/AoC2023Day02.txt")
    .trim()
    .lines()
    .filter_map(|line| {
      let (game, sets) = line.split_once(":")?;
      let game_id = game
        .trim_start_matches("Game")
        .trim()
        .parse::<usize>()
        .ok()?;

      let mut game_record = GameRecord::new(game_id, 0, 0, 0);

      sets.split(";").for_each(|set| {
        set.split(",").for_each(|item| {
          let mut item_splitted = item.split_whitespace();
          let count = item_splitted
            .next()
            .and_then(|s| s.parse().ok())
            .unwrap_or(0);

          match item_splitted.next() {
            Some("red") => game_record.update_max_red(count),
            Some("green") => game_record.update_max_green(count),
            Some("blue") => game_record.update_max_blue(count),
            _ => {}
          }
        });
      });

      Some(game_record.max_red * game_record.max_green * game_record.max_blue)
    })
    .sum::<usize>();

  println!("Part 1: {part_1}");
  println!("Part 2: {part_2}");
}
