use std::collections::HashMap;

fn main() {
  let mut scores = HashMap::new();
  scores.insert("A", 10);
  scores.insert("B", 7);

  for (k,v) in &scores {
    println!("{}: {}", k, v);
  }

  let v = scores.get("A");
  match v {
      None => println!("There is no A"),
      Some(v) => println!("Found for A: {}", v)
  }

  scores.entry("C").or_insert(12);
  scores.entry("C").or_insert(43);

  println!("{:?}", scores);

  // Count number of occurrences of word (neat trick)

  let text = "hello this is a hello";

  let mut word_count = HashMap::new();

  for word in text.split_whitespace() {
    let count = word_count.entry(word).or_insert(0);
    *count += 1;
  }

  println!("{:?}", word_count);
}