fn main() {
  let v = 1_006 + 0xff + 0o77 + 0b1111_1111;
  assert!(v == 1597);
}

//0o77 = 63
//0xff = 255
//binary = 255