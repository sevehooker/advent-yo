use std::fs::File;
use std::io::Read;
use std::iter::Filter;

fn main() {
  let mut file = match File::open("resource/day6.txt") {
    Ok(file) => file,
    Err(_) => panic!("no such file"),
  };

  let mut file_contents = String::new();
  
  file.read_to_string(&mut file_contents)
    .ok()
    .expect("file fail");

  let input = file_contents
    .split("\r\n")
    .map(|s: &str| s)
    .collect::<Vec<&str>>();

  let space = map_system(input);

  println!("{:?}", space);
  println!("All the orbzits: {}", count_orbits(space));
}

fn map_system(input: Vec<&str>) -> Vec<Vec<&str>> {
  let mut space: Vec<Vec<&str>> = Vec::new();

  for line in input {
    let orbit = line.split(')').collect::<Vec<&str>>();

    if space.len() == 0 {
      space.push(vec!(orbit[0], orbit[1]));
    } else {
      for (i, system) in space.iter().enumerate() {
        let orbited_pos = system.iter().position(|&o| o == orbit[0]);

        match orbited_pos {
          Some(pos) => {
            if pos+1 < system.len() {
              space.push(vec!(orbit[0], orbit[1]));
            } else {
              space[i].insert(pos+1, orbit[1]);
            }
            break;
          },
          None => {}
        }
        
        if i == space.len()-1 {
          space.push(vec!(orbit[0], orbit[1]));
          break;
        }

      }
    }
  }

  return space;
}

fn count_orbits(space: Vec<Vec<&str>>) -> u32 {
  let mut count = 0;
  for system in space.iter() {
    // let mut another_system: usize = 0;
    
    // for starting in space.iter().filter(|orbiting| orbiting[0] != system[0]) {
    //   match starting.iter().position(|&o| o == system[0]) {
    //     Some(pos) => { 
    //       another_system = pos;
    //       break; },
    //     None => {}
    //   }
    // };

    // count += system.iter().enumerate().skip(1).fold(0, |acc, (pos, _)| acc + pos + another_system);
    count += system.iter().enumerate().skip(1).fold(0, |acc, (pos, _)| {
      return acc + pos + system_distance(&system, space.iter().filter(|orbiting| orbiting[0] != system[0]).collect());
    });
  }
  return count as u32;
}

fn system_distance(system: &Vec<&str>, space: Vec<&Vec<&str>>) -> usize {
  for starting in space {
    match starting.iter().position(|&o| o == system[0]) {
      Some(pos) => {
        return system_distance(&system, space.iter().filter(|orbiting| orbiting[0] != system[0]).collect()) + pos;
      },
      None => {}
    }
  };

  return 0 as usize;
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_day6() {
    let orbits = vec!(
      "COM)B",
      "B)C",
      "C)D",
      "D)E",
      "E)F",
      "B)G",
      "G)H",
      "D)I",
      "E)J",
      "J)K",
      "K)L"
    );

    let system = map_system(orbits);

    assert_eq!(count_orbits(system), 424);
  }
}