use std::fs::File;
use std::io::{BufReader,BufRead};

fn main() {
    let file = File::open("resource/mass.txt").unwrap();
    let mut numbers = Vec::new();

    for line in BufReader::new(file).lines() {
        numbers.push(line.unwrap().parse::<i32>().unwrap());
    }
    
    println!("{}", numbers.iter().fold(0, |sum, fuel| sum + ((fuel/3)-2)));
    println!("{}", part_1(&numbers));
    // println!("{}", part_2(&numbers));
}

fn part_1(numbers: &[i32]) -> i32 {
    let mut sum = 0;

    for mass in numbers {
        let fuel = mass / 3;
        sum += fuel - 2;
    }

    return sum;
}

fn part_2(numbers: &[i32]) -> i32 {
    let mut sum = 0;

}