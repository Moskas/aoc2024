use std::fs;

fn get_input() -> Vec<String> {
  fs::read_to_string("input.txt")
    .expect("No input file provided")
    .lines()
    .map(|s| s.to_string())
    .collect::<Vec<String>>()
}

fn is_safe(report: &Vec<i32>) -> bool {
  let mut increasing = true;
  let mut decreasing = true;

  for i in 1..report.len() {
    let diff = (report[i] - report[i - 1]).abs();

    // Check if the difference is within the allowed range
    if diff < 1 || diff > 3 {
      return false; // If the difference is not within the valid range, it's unsafe
    }

    // Check if the report is increasing or decreasing
    if report[i] > report[i - 1] {
      decreasing = false;
    } else if report[i] < report[i - 1] {
      increasing = false;
    }
  }

  // The report is safe if it's either strictly increasing or strictly decreasing
  increasing || decreasing
}

fn can_be_safe_with_one_removal(report: &Vec<i32>) -> bool {
  // Try removing one element and check if the remaining report is safe
  for i in 0..report.len() {
    let mut new_report = report.clone();
    new_report.remove(i);

    if is_safe(&new_report) {
      return true; // If removing this level makes the report safe, return true
    }
  }
  false // No single removal made it safe
}

fn main() {
  let input = get_input();
  // Part I
  let mut levels: Vec<Vec<i32>> = Vec::new();
  for entry in input {
    levels.push(
      entry
        .split_whitespace()
        .collect::<Vec<&str>>()
        .iter()
        .map(|s| s.parse::<i32>().expect("Not a number"))
        .collect::<Vec<i32>>(),
    );
  }

  let safe_reports_count = levels.iter().filter(|&report| is_safe(report)).count();

  println!("Number of safe reports: {}", safe_reports_count);

  let safe_reports_count = levels
    .iter()
    .filter(|&report| is_safe(report) || can_be_safe_with_one_removal(report))
    .count();

  println!(
    "Number of safe reports (with dampener): {}",
    safe_reports_count
  );
}
