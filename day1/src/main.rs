use std::fs::File;
use std::io::{BufReader,BufRead};

fn main() {
  let file = File::open("resource/mass.txt").unwrap();
  let numbers: Vec<u32> = BufReader::new(file).lines().map(|v| v.unwrap().parse::<u32>().unwrap()).collect();
  
  let total_fuel = |sum:u32, fuel: &u32| {
    let mut fuel_fuel = *fuel;
    let mut total = 0;

    while fuel_fuel > 8 {
      fuel_fuel = (fuel_fuel/3)-2;
      total += fuel_fuel;
    }

    return sum + total;
  };

  println!("{}", numbers.iter().fold(0, total_fuel));
}

#[cfg(test)]
mod tests {
  #[test]
  fn test_day1() {
    let mut masses = Vec::new();
    masses.push(mass::Mass::new(1969));

    assert_eq!(masses.iter().map(|m| m.burn_baby()).sum(), 966);
  }
}