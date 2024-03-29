use std::cell::Cell;
use std::fs;

fn main() {
  let input = fs::read_to_string("resource/day2.txt").expect("file");

  let numbers: Vec<Cell<u32>> = input
    .trim()
    .split(',')
    .map(|n| Cell::new(n.parse::<u32>().unwrap()))
    .collect();

  for i in 0..99 {
    for j in 0..99 {
      let cur_numbers = numbers.to_vec();
      cur_numbers[1].set(i as u32);
      cur_numbers[2].set(j as u32);
      let result = op_it_up(cur_numbers);

      if result[0].get() == 19690720 {
        panic!(
          "at the disco: {}",
          (result[1].get() * 100) + result[2].get()
        );
      }
    }
  }
  let result = op_it_up(numbers);

  println!("{:?}", result[0].get());
}

fn op_it_up(numbers: Vec<Cell<u32>>) -> Vec<Cell<u32>> {
  for (i, value) in numbers.iter().enumerate().step_by(4) {
    match value.get() {
      1 => numbers[numbers[i + 3].get() as usize].set(
        numbers[numbers[i + 1].get() as usize].get() + numbers[numbers[i + 2].get() as usize].get(),
      ),
      2 => numbers[numbers[i + 3].get() as usize].set(
        numbers[numbers[i + 1].get() as usize].get() * numbers[numbers[i + 2].get() as usize].get(),
      ),
      99 => break,
      _ => break,
    }
  }

  return numbers;
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_part1() {
    assert_eq!(
      op_it_up(vec![
        Cell::new(1),
        Cell::new(0),
        Cell::new(0),
        Cell::new(0),
        Cell::new(99)
      ]),
      vec![
        Cell::new(2),
        Cell::new(0),
        Cell::new(0),
        Cell::new(0),
        Cell::new(99)
      ]
    );
    assert_eq!(
      op_it_up(vec![
        Cell::new(2),
        Cell::new(3),
        Cell::new(0),
        Cell::new(3),
        Cell::new(99)
      ]),
      vec![
        Cell::new(2),
        Cell::new(3),
        Cell::new(0),
        Cell::new(6),
        Cell::new(99)
      ]
    );
    assert_eq!(
      op_it_up(vec![
        Cell::new(2),
        Cell::new(4),
        Cell::new(4),
        Cell::new(5),
        Cell::new(99),
        Cell::new(0)
      ]),
      vec![
        Cell::new(2),
        Cell::new(4),
        Cell::new(4),
        Cell::new(5),
        Cell::new(99),
        Cell::new(9801)
      ]
    );
    assert_eq!(
      op_it_up(vec![
        Cell::new(1),
        Cell::new(1),
        Cell::new(1),
        Cell::new(4),
        Cell::new(99),
        Cell::new(5),
        Cell::new(6),
        Cell::new(0),
        Cell::new(99)
      ]),
      vec![
        Cell::new(30),
        Cell::new(1),
        Cell::new(1),
        Cell::new(4),
        Cell::new(2),
        Cell::new(5),
        Cell::new(6),
        Cell::new(0),
        Cell::new(99)
      ]
    );
    assert_eq!(
      op_it_up(vec![
        Cell::new(1),
        Cell::new(9),
        Cell::new(10),
        Cell::new(3),
        Cell::new(2),
        Cell::new(3),
        Cell::new(11),
        Cell::new(0),
        Cell::new(99),
        Cell::new(30),
        Cell::new(40),
        Cell::new(50)
      ]),
      vec![
        Cell::new(3500),
        Cell::new(9),
        Cell::new(10),
        Cell::new(70),
        Cell::new(2),
        Cell::new(3),
        Cell::new(11),
        Cell::new(0),
        Cell::new(99),
        Cell::new(30),
        Cell::new(40),
        Cell::new(50)
      ]
    );
  }
}
