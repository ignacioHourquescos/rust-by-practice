fn main() {
  let mut sum = 0;
  // modifying adding "=" in for to end result -5 in order to assert
  for i in -3..=1 {
      sum += i
  }

  assert!(sum == -5);
  // converted c to u8, string to base integer
  for c in 'a'..='z' {
      println!("{}",c as u8);
  }
}

