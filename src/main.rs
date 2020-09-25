use std::{thread, time};

fn main() {
  let mut count: i32 = 10;

  println!("Launch commencing in:");

  while count > 0 {
    println!("{}", count);

    thread::sleep(time::Duration::from_secs(1));

    count = count - 1;
  }

  println!("Blast off!")
}
