use std::cell::Cell;
use std::fs;

#[macro_use] extern crate scan_rules;

enum ParameterMode {
  Position = 0,
  Immediate
}

fn main() {
  let input = fs::read_to_string("resource/day5.txt").expect("file");

  let numbers: Vec<Cell<i32>> = input
    .trim()
    .split(',')
    .map(|n| Cell::new(n.parse::<i32>().unwrap()))
    .collect();

  let result = op_it_up(numbers);
}

fn op_it_up(numbers: Vec<Cell<i32>>) -> Vec<Cell<i32>> {
  let mut i = 0;
  while i < numbers.len() {
    let cur_code = numbers[i].get();
    let mut num_params = 1;
    match cur_code % 100 {
      1 => {
        num_params = 4;
        let sum = get_op_value(cur_code / 100 % 10, numbers[i+1].get(), &numbers) + get_op_value(cur_code / 1000 % 10, numbers[i+2].get(), &numbers);
        numbers[numbers[i+3].get() as usize].set(sum);
      },
      2 => {
        num_params = 4;
        let sum = get_op_value(cur_code / 100 % 10, numbers[i+1].get(), &numbers) * get_op_value(cur_code / 1000 % 10, numbers[i+2].get(), &numbers);
        numbers[numbers[i+3].get() as usize].set(sum);
      },
      3 => { 
        println!("What do?");
        readln! {
          (let num: i32) => {
            num_params = 2;
            numbers[numbers[i+1].get() as usize].set(num);
          }
        }
      },
      4 => {
        num_params = 2;
        println!("{}", get_op_value(cur_code / 100 % 10, numbers[i+1].get(), &numbers));
      },
      5 => {
        num_params = 3;
        if get_op_value(cur_code / 100 % 10, numbers[i+1].get(), &numbers) != 0 {
          num_params = 0;
          i = get_op_value(cur_code / 1000 % 10, numbers[i+2].get(), &numbers) as usize;
        }
      },
      6 => {
        num_params = 3;
        if get_op_value(cur_code / 100 % 10, numbers[i+1].get(), &numbers) == 0 {
          num_params = 0;
          i = get_op_value(cur_code / 1000 % 10, numbers[i+2].get(), &numbers) as usize;
        }
      },
      7 => {
        num_params = 4;
        if get_op_value(cur_code / 100 % 10, numbers[i+1].get(), &numbers) < get_op_value(cur_code / 1000 % 10, numbers[i+2].get(), &numbers) {
          numbers[numbers[i+3].get() as usize].set(1);
        } else {
          numbers[numbers[i+3].get() as usize].set(0);
        }
      },
      8 => {
        num_params = 4;
        if get_op_value(cur_code / 100 % 10, numbers[i+1].get(), &numbers) == get_op_value(cur_code / 1000 % 10, numbers[i+2].get(), &numbers) {
          numbers[numbers[i+3].get() as usize].set(1);
        } else {
          numbers[numbers[i+3].get() as usize].set(0);
        }
      },
      99 => break,
      _ => panic!("{}", cur_code)
    }
    i += num_params;
  }

  return numbers;
}

fn get_op_value(mode: i32, value: i32, numbers: &Vec<Cell<i32>>) -> i32 {
  match mode {
    mode if mode == ParameterMode::Position as i32 => return numbers[value as usize].get(),
    mode if mode == ParameterMode::Immediate as i32 => return value,
    _ => panic!("Oh noes parameter hoes: {}", mode)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_1() {
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

  #[test]
  fn test_2() {
    assert_eq!(
      op_it_up(vec![
        Cell::new(1002),
        Cell::new(4),
        Cell::new(3),
        Cell::new(4),
        Cell::new(33)
      ]),
      vec![
        Cell::new(1002),
        Cell::new(4),
        Cell::new(3),
        Cell::new(4),
        Cell::new(99)
      ]
    );
  }

  // Dunno how 🤷‍♂️
  // #[test]
  // fn test_input() {
  //   assert_eq!(
  //     op_it_up(vec![
  //       Cell::new(3),
  //       Cell::new(0),
  //       Cell::new(4),
  //       Cell::new(0),
  //       Cell::new(99)
  //     ]),
  //     vec![
  //       Cell::new(3),
  //       Cell::new(0),
  //       Cell::new(4),
  //       Cell::new(0),
  //       Cell::new(99)
  //     ]
  //   );
  // }
}
