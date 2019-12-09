fn main() {
  let low = 109165;
  let high = 576723;

  println!("{}", (low..high).fold(0, |count, num| count + if valid_num(num) { 1 } else { 0 }));
}

fn valid_num(num: u32) -> bool {
  let mut has_double = false;
  let multiples = [10,100,1000,10000,100000];

  for tens in multiples.iter() {
    let cur_digit = (num / tens) % 10;
    let prev_digit = (num / (tens / 10)) % 10;

    if cur_digit > prev_digit {
      return false;
    }

    if tens == &10 {
      has_double = has_double || (
        cur_digit == prev_digit &&
        cur_digit != ((num / (tens*10)) % 10));
    } else if tens == &100000 {
      has_double = has_double || (
        cur_digit == prev_digit &&
        cur_digit != ((num / (tens/100)) % 10));
    } else {
      has_double = has_double || (
        cur_digit == prev_digit &&
        cur_digit != ((num / (tens*10)) % 10) &&
        cur_digit != ((num / (tens/100)) % 10)
      );
    }
  }

  return has_double;
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn is_inc() {
    assert_eq!(valid_num(223450), false);
  }

  #[test]
  fn no_double() {
    assert_eq!(valid_num(123789), false);
  }

  #[test]
  fn no_longer_valid() {
    assert_eq!(valid_num(111111), false);
  }

  #[test]
  fn end_big_group() {
    assert_eq!(valid_num(123444), false);
  }

  
  #[test]
  fn begin_big_group() {
    assert_eq!(valid_num(444567), false);
  }

  #[test]
  fn is_valid() {
    assert_eq!(valid_num(112233), true);
  }

  #[test]
  fn big_group_valid() {
    assert_eq!(valid_num(111122), true);
  }
}
