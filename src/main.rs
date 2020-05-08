use std::{sync::mpsc, thread, time::Duration};

fn main() {
  exhibit1();
  exhibit2();
}

fn exhibit1() {
  let v = vec![1, 2, 3];
    let x= vec![4, 5, 6];

    let (producer, consumer) = mpsc::channel();

    let handle = thread::spawn(move || {
        for i in v.iter() {
            println!("Printing thread: {}", i);
        }
        producer.send("Thread Done").unwrap();
    });

    for i in x.iter() {
        println!("Printing main: {}", i);
    }

    let message = consumer.recv().unwrap();
    println!("From thread: {}", message);

    handle.join().unwrap();
}

fn exhibit2() {
  let (tx, rx) = mpsc::channel();

  let tx_another = tx.clone();

  thread::spawn(move || {
    let vals = vec![
      "one",
      "two",
      "three"
    ];

    for val in vals{
      println!("Sending {}", val);
      tx.send(val).unwrap();
      thread::sleep(Duration::from_secs(1));
    }
  });

  thread::spawn(move || {
    let vals = vec![
      "four",
      "five",
      "fix"
    ];

    for val in vals{
      println!("Sending {}", val);
      tx_another.send(val).unwrap();
      thread::sleep(Duration::from_secs(1));
    }
  });

  for recv in rx {
    println!("Receiving: {}", recv);
  }
}