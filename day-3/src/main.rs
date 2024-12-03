use regex::Regex;
use std::fs;

fn get_input() -> Vec<String> {
  fs::read_to_string("input.txt")
    .expect("No input file provided")
    .lines()
    .map(|s| s.to_string())
    .collect::<Vec<String>>()
}

fn main() {
  let input = get_input().concat();
  // Part I
  let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
  let mut total = 0;
  for cap in re.captures_iter(&input) {
    let num1: i32 = cap[1].parse().unwrap();
    let num2: i32 = cap[2].parse().unwrap();
    total += num1 * num2;
  }

  println!("Total result of mul instructions: {}", total);

  // Part II
  // let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
  // Regex to match the do() and don't() instructions
  //let do_re = Regex::new(r"do\(\)").unwrap();
  //let dont_re = Regex::new(r"don't\(\)").unwrap();

  //let mut sum = 0;
  //let mut mul_enabled = true; // mul instructions are enabled by default

  //// Iterate over the entire input string
  // let mut i = 0;
  // while i < input.len() {
  //   // Check for do() or don't() instructions
  //   if let Some(cap) = do_re.find(&input[i..]) {
  //     mul_enabled = true; // Enable multiplication
  //     i += cap.end(); // Move past the "do()" instruction
  //   } else if let Some(cap) = dont_re.find(&input[i..]) {
  //     mul_enabled = false; // Disable multiplication
  //     i += cap.end(); // Move past the "don't()" instruction
  //   } else {
  //     // Check for mul(X,Y) instructions
  //     if let Some(cap) = re.captures(&input[i..]) {
  //       if mul_enabled {
  //         let x: i32 = cap[1].parse().unwrap();
  //         let y: i32 = cap[2].parse().unwrap();
  //         sum += x * y;
  //       }
  //       i += cap.get(0).unwrap().end(); // Skip over this mul instruction
  //     } else {
  //       i += 1; // Move to next character if no match
  //     }
  //   }
  // }

  //// Print the total sum
  // println!("Total sum of enabled multiplications: {}", sum);
}
