fn main() {
  let v1 = 247_u8 + 8;
  let v2 = i8::checked_add(119, 8).unwrap();
  println!("{},{}",v1,v2);
}

// reached max of i8 247+8 =255
// idem for i8 =>127