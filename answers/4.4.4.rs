//This allows a program to terminate immediately and provide feedback to the caller of the program.
fn never_return_fn() -> ! {
  panic!()
}


//Indicates unimplemented code by panicking with a message of “not implemented”.
fn never_return_fn() -> ! {
  //using panic to e
  unimplemented!()
}


//***need to dive deeper into this one
fn never_return_fn() -> ! {
  loop {
      std::thread::sleep(std::time::Duration::from_secs(1))
  }
}