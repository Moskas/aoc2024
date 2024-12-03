use regex::Regex;
use std::fs;

fn get_input() -> String {
  fs::read_to_string("input.txt").expect("No input file provided")
}

fn main() {
  let input = get_input();

  let re = Regex::new(r"mul\((\d+),(\d+)\)|(do\(\))|(don't\(\))").unwrap();

  let mut total1 = 0;
  let mut total2 = 0;
  let mut enabled = true;

  for cap in re.captures_iter(&input) {
    if cap.get(3).is_some() {
      // Match "do()"
      enabled = true;
    } else if cap.get(4).is_some() {
      // Match "don't()"
      enabled = false;
    } else if let (Some(a), Some(b)) = (cap.get(1), cap.get(2)) {
      // Match "mul(a, b)"
      let a: i32 = a.as_str().parse().unwrap();
      let b: i32 = b.as_str().parse().unwrap();
      let x = a * b;
      total1 += x;
      if enabled {
        total2 += x;
      }
    }
  }

  println!("Total result of mul instructions: {}", total1);
  println!("Total result of only enabled mul instructions: {}", total2);
}
