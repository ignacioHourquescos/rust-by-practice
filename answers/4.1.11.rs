fn main() {

  //compiler infers type of variable u32 =>"3"
  assert!(1u32 + 2 == 3);

  //"-1" => MIN i32 < -1
  assert!(1i32 - 2 == -1);

  //changed to i8, u8 min is 0
  assert!(1i8 - 2 == -1);

  //simple multipliacation
  assert!(3 * 50 == 150);

  //sadded f32 flaoting point
  assert!(9.6_f32 / 3.2_f32 == 3); 

  assert!(24 % 5 == 4);
  // Short-circuiting boolean logic
  assert!(true && false == false);
  assert!(true || false == true);
  assert!(!true == false);

  // Bitwise operations
  println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
  println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
  println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
  println!("1 << 5 is {}", 1u32 << 5);
  println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);
}