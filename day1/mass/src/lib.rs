pub struct Mass {
  fuel: u32,
}

impl Mass {
  pub fn new(mass: u32) -> Mass {
    Mass { fuel: mass }
  }

  pub fn burn_baby(&mut self) -> u32 {
    let mut total = 0;
    for cur_fuel in self.next() {
      total += cur_fuel;
    }

    return total;
  }
}

impl Iterator for Mass {
  type Item = u32;

  fn next(&mut self) -> Option<Self::Item> {
    if self.fuel > 8 {
      self.fuel = (self.fuel / 3) - 2;
      return Some(self.fuel);
    } else {
      return None;
    }
  }
}

#[cfg(test)]
mod tests {
  #[test]
  fn it_works() {
    assert_eq!(2 + 2, 4);
  }
}
