use std::iter::once;

#[derive(PartialEq, Debug)]
enum Operator {
  Add,
  Multiply,
}

impl From<char> for Operator {
  fn from(c: char) -> Self {
    match c {
      '+' => Operator::Add,
      '*' => Operator::Multiply,
      _ => panic!(),
    }
  }
}

trait CharExt {
  fn is_operator(&self) -> bool;
}

impl CharExt for char {
  fn is_operator(&self) -> bool {
    matches!(self, '+' | '*')
  }
}

#[derive(PartialEq, Debug)]
struct Column {
  operands: Vec<usize>,
  operator: Operator,
}

impl Column {
  fn new(operands: Vec<usize>, operator: Operator) -> Self {
    Column { operands, operator }
  }

  fn evaluate(&self) -> usize {
    match self.operator {
      Operator::Add => self.operands.iter().sum(),
      Operator::Multiply => self.operands.iter().product(),
    }
  }
}

fn main() {
  let input = include_str!("input/AoC2025Day06.txt")
    .lines()
    .filter(|line| !line.is_empty())
    .collect::<Vec<_>>();
  let columns_1 = parse_columns_part_1(&input);
  let columns_2 = parse_columns_part_2(&input);

  println!("Part 1: {}", evaluate(&columns_1));
  println!("Part 2: {}", evaluate(&columns_2));
}

fn evaluate(columns: &[Column]) -> usize {
  columns.iter().map(Column::evaluate).sum()
}

fn parse_columns_part_1(input: &[&str]) -> Vec<Column> {
  let input = input
    .iter()
    .map(|line| line.split_whitespace().collect::<Vec<_>>())
    .collect::<Vec<_>>();
  let (operand_rows, operator_row) = input.split_at(input.len() - 1);
  let operator_row = operator_row.first().unwrap();

  operator_row
    .iter()
    .enumerate()
    .map(|(i, op)| {
      let operands = operand_rows
        .iter()
        .map(|row| row.get(i).unwrap().parse().unwrap())
        .collect();
      let operator = op.chars().next().unwrap().into();

      Column::new(operands, operator)
    })
    .collect()
}

fn parse_columns_part_2(input: &[&str]) -> Vec<Column> {
  let input = input
    .iter()
    .map(|line| line.chars().collect::<Vec<_>>())
    .collect::<Vec<_>>();
  let (operand_rows, operator_row) = input.split_at(input.len() - 1);
  let operator_row = operator_row.first().unwrap();
  let operators_indices = operator_row
    .iter()
    .enumerate()
    .filter_map(|(i, op)| (op.is_operator()).then_some(i))
    .collect::<Vec<_>>();
  let operator_indices = operators_indices
    .windows(2)
    .map(|w| w[0]..(w[1] - 1))
    .chain(once(*operators_indices.last().unwrap()..operator_row.len()))
    .collect::<Vec<_>>();

  operator_indices
    .iter()
    .map(|range| {
      let operands = range
        .clone()
        .into_iter()
        .map(|j| {
          operand_rows
            .iter()
            .map(move |row| row.get(j).unwrap())
            .collect::<String>()
        })
        .map(|s| s.trim().parse().unwrap())
        .collect::<Vec<_>>();
      let operator = (*operator_row.get(range.start).unwrap()).into();

      Column::new(operands, operator)
    })
    .collect::<Vec<_>>()
}

#[cfg(test)]
mod tests {
  use super::*;

  const TEST_INPUT: &str = "123 328  51 64 \n 45 64  387 23 \n  6 98  215 314\n*   +   *   +  \n";

  #[test]
  fn test_into_operator() {
    assert_eq!(Operator::from('+'), Operator::Add);
    assert_eq!(Operator::from('*'), Operator::Multiply);

    let add: Operator = '+'.into();
    let multiply: Operator = '*'.into();
    assert_eq!(add, Operator::Add);
    assert_eq!(multiply, Operator::Multiply);
  }

  #[test]
  fn test_column_evaluate() {
    let column = Column::new(vec![123, 45, 6], Operator::Multiply);
    assert_eq!(column.evaluate(), (123 * 45 * 6));

    let column = Column::new(vec![328, 64, 98], Operator::Add);
    assert_eq!(column.evaluate(), (328 + 64 + 98));

    let column = Column::new(vec![51, 387, 215], Operator::Multiply);
    assert_eq!(column.evaluate(), (51 * 387 * 215));

    let column = Column::new(vec![64, 23, 314], Operator::Add);
    assert_eq!(column.evaluate(), (64 + 23 + 314));
  }

  #[test]
  fn test_parse_columns_part_1() {
    let input = TEST_INPUT
      .lines()
      .map(str::trim)
      .filter(|line| !line.is_empty())
      .collect::<Vec<_>>();
    let columns = parse_columns_part_1(&input);

    assert_eq!(columns.len(), 4);
    assert_eq!(
      columns[0],
      Column::new(vec![123, 45, 6], Operator::Multiply)
    );
    assert_eq!(columns[1], Column::new(vec![328, 64, 98], Operator::Add));
    assert_eq!(
      columns[2],
      Column::new(vec![51, 387, 215], Operator::Multiply)
    );
    assert_eq!(columns[3], Column::new(vec![64, 23, 314], Operator::Add));
  }

  #[test]
  fn test_parse_columns_part_2() {
    let input_2 = TEST_INPUT
      .lines()
      .filter(|line| !line.is_empty())
      .collect::<Vec<_>>();
    let columns = parse_columns_part_2(&input_2);

    assert_eq!(columns.len(), 4);
    assert_eq!(
      columns[0],
      Column::new(vec![1, 24, 356], Operator::Multiply)
    );
    assert_eq!(columns[1], Column::new(vec![369, 248, 8], Operator::Add));
    assert_eq!(
      columns[2],
      Column::new(vec![32, 581, 175], Operator::Multiply)
    );
    assert_eq!(columns[3], Column::new(vec![623, 431, 4], Operator::Add));
  }

  #[test]
  fn test_part_1() {
    let input = TEST_INPUT
      .lines()
      .map(str::trim)
      .filter(|line| !line.is_empty())
      .collect::<Vec<_>>();
    let columns = parse_columns_part_1(&input);

    assert_eq!(evaluate(&columns), (33210 + 490 + 4243455 + 401));
  }

  #[test]
  fn test_part_2() {
    let input = TEST_INPUT
      .lines()
      .filter(|line| !line.is_empty())
      .collect::<Vec<_>>();
    let columns = parse_columns_part_2(&input);

    assert_eq!(evaluate(&columns), (1058 + 3253600 + 625 + 8544));
  }
}
