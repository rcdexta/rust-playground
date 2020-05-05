use std::{io::ErrorKind, fs::File};

fn main() {
  let f = File::open("hello.txt");

  let file = match f {
      Ok(file) => file,
      Err(err) => match err.kind() {
          ErrorKind::NotFound => match File::create("hello.txt") {
            Ok(fc) => fc,
            Err(e) => panic!("Problem creating the file: {:?}", e),
        },
        x => panic!("Problem opening the file: {:?}", x)
      }
  };

  let another_file = File::open("a.txt").unwrap();
  let a_file = File::open("b.txt").expect("Failed to find b.txt");
  
}