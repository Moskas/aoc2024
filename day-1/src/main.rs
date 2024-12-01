use std::collections::HashMap;
use std::fs;

fn get_input() -> Vec<String> {
  fs::read_to_string("input.txt")
    .expect("No input file provided")
    .lines()
    .map(|s| s.to_string())
    .collect::<Vec<String>>()
}

fn main() {
  let input = get_input();
  // Part I
  let (mut left, mut right) = (vec![], vec![]);

  for line in input {
    if let [f, s] = line.split_whitespace().take(2).collect::<Vec<&str>>()[..] {
      let f_num = f.parse::<i32>().unwrap();
      let s_num = s.parse::<i32>().unwrap();

      left.push(f_num);
      right.push(s_num);
    }
  }

  left.sort_unstable();
  right.sort_unstable();

  let sum: i32 = left
    .iter()
    .enumerate()
    .map(|(i, l)| (l - &right[i]).abs())
    .sum();
  println!("Sum of distances: {}", sum);

  // Part II
  let mut right_count: HashMap<i32, i32> = HashMap::new();
  for num in right {
    *right_count.entry(num).or_insert(0) += 1;
  }

  let similarity_score: i32 = left
    .iter()
    .map(|&num| num * right_count.get(&num).unwrap_or(&0))
    .sum();

  println!("Similarity score: {}", similarity_score);
}
