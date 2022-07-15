fn main() {
  let v = {
      let mut x = 1;
      //added semicolon so () is assigned
      x += 2;
  };

  assert_eq!(v, ());
}


fn main() {
  let v = {
      let mut x = 1;
      x += 2;
      //assigned value of x to v = 3
      x
  };

  assert_eq!(v, 3);

  println!("Success!");
}