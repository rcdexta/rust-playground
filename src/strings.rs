fn main() {
  let s1 = String::from("new String");
  let s2 = "Another String".to_string();

  println!("{}, {}", s1, s2);

  let s3 = s1 + &s2;

  let s = format!("{} && {}", s2, s3);
  println!("{} + {} = {}", s2, s3, s);


  println!("By iterating as chars!:");
  for c in s.chars() {
    print!("{}", c);
  }

}