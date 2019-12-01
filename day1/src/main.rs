use std::fs::File;
use std::io::{BufReader,BufRead};

// struct Fuel {
//     amount: i32
// }

// impl Iterator for Fuel {
//     type Item = i32;

//     fn next(&mut self) -> Option<Self::Item> {
//         let mut total = 0;
//         let mut cur_amount = self.amount;

//         while cur_amount > 1 {
//             cur_amount = (cur_amount/3)-2;
//             total += cur_amount;
//         }

//         Some(total);
//     }
// }

fn main() {
  let file = File::open("resource/mass.txt").unwrap();
  let mut numbers = Vec::new();

  let total_fuel = move |sum:i32, fuel: &i32| {
    let mut fuel_fuel = *fuel;
    let mut total = 0;

    while fuel_fuel > 8 {
      fuel_fuel = (fuel_fuel/3)-2;
      total += fuel_fuel;
    }

    return sum + total;
  };

  for line in BufReader::new(file).lines() {
    numbers.push(line.unwrap().parse::<i32>().unwrap());
  }
  
  println!("{}", numbers.iter().fold(0, total_fuel));
}

#[cfg(test)]
mod tests {
  #[test]
  fn test_day1() {
    let mut numbers = Vec::new();
    numbers.push(1969);

    let total_fuel = move |sum:i32, mass: &i32| {
      let mut fuel_mass = *mass;
      let mut total = 0;
  
      while fuel_mass > 8 {
        fuel_mass = (fuel_mass/3)-2;
        println!("{}", fuel_mass);

        total += fuel_mass;
      }
  
      return sum + total;
    };

    assert_eq!(numbers.iter().fold(0, total_fuel), 966);
  }
}