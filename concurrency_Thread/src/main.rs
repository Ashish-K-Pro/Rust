// concurrency-bp

use std::{thread, time};

fn main() {
  let duration = time::Duration::from_millis(3000);

  println!("Main thread");

  let handle  = thread::spawn(move || {
    println!("Inner thread 1");

    let handle2 = thread::spawn( move || {
      println!("Inner thread 2");
      thread::sleep(duration);
    });

    handle2.join().unwrap();
    thread::sleep(duration);
  });

  handle.join().unwrap();

  thread::sleep(duration);
}
