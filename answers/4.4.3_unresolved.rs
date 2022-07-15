//1 remove print in order to avoid error. 
// With panic! you provide a message that describes the bug and the language then constructs an error with that message,
fn main() {
  never_return();
}

fn never_return() -> ! {
  // Implement this function, don't modify the fn signatures
   panic!("I return nothing!")
  
}


//***have to figure this out
fn main() {
  never_return();
}

use std::thread;
use std::time;

fn never_return() -> ! {
  // implement this function, don't modify fn signatures
  loop {
      println!("I return nothing");
      // sleeping for 1 second to avoid exhausting the cpu resoucre
      thread::sleep(time::Duration::from_secs(1))
  }
}