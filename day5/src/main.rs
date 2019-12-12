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

  println!("{:?}", result[0].get());
}

fn op_it_up(numbers: Vec<Cell<i32>>) -> Vec<Cell<i32>> {
  let mut codes = numbers.iter().map(|c| c.get());
  loop {
    let cur_code = codes.next().unwrap();
    match cur_code % 100 {
      1 => {
        let sum = get_op_value(cur_code / 100 % 10, codes.next().unwrap(), &numbers) + get_op_value(cur_code / 1000 % 10, codes.next().unwrap(), &numbers);
        numbers[codes.next().unwrap() as usize].set(sum);
      },
      2 => {
        let product = get_op_value(cur_code / 100 % 10, codes.next().unwrap(), &numbers) * get_op_value(cur_code / 1000 % 10, codes.next().unwrap(), &numbers);
        numbers[codes.next().unwrap() as usize].set(product);
      },
      3 => { 
        println!("What do?");
        readln! {
          (let num: i32) => {
            numbers[codes.next().unwrap() as usize].set(num);
          }
        }
      },
      4 => println!("{}", get_op_value(cur_code / 100 % 10, codes.next().unwrap(), &numbers)),
      5 => {
        if get_op_value(cur_code / 100 % 10, codes.next().unwrap(), &numbers) != 0 {
          let jumpTo = get_op_value(cur_code / 1000 % 10, codes.next().unwrap(), &numbers);
          codes.enumerate().skip_while(|(_, i)| i < &jumpTo);
        } else {
          codes.next();
        }
      },
      6 => {
        if get_op_value(cur_code / 100 % 10, codes.next().unwrap(), &numbers) == 0 {
          let jumpTo = get_op_value(cur_code / 1000 % 10, codes.next().unwrap(), &numbers);
          codes.enumerate().skip_while(|(_, i)| i < &jumpTo);
        } else {
          codes.next();
        }
      },
      7 => {
        if get_op_value(cur_code / 100 % 10, codes.next().unwrap(), &numbers) < get_op_value(cur_code / 100 % 10, codes.next().unwrap(), &numbers) {
          numbers[codes.next().unwrap() as usize].set(1);
        } else {
          numbers[codes.next().unwrap() as usize].set(0);
        }
      },
      8 => {
        if get_op_value(cur_code / 100 % 10, codes.next().unwrap(), &numbers) == get_op_value(cur_code / 100 % 10, codes.next().unwrap(), &numbers) {
          numbers[codes.next().unwrap() as usize].set(1);
        } else {
          numbers[codes.next().unwrap() as usize].set(0);
        }
      }
      99 => break,
      _ => panic!("{}", cur_code)
    }
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
