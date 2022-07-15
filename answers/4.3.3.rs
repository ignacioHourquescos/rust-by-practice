fn main() {
  let s = sum(1 , 2);
  assert_eq!(s, 3);
}

fn sum(x: i32, y: i32) -> i32 {
  //remove semicolon for resutl to be asigned to s
  x + y
}