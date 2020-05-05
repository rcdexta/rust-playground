trait Summary {
  fn summarize(&self) -> String {
    "Summary comes here".to_string()
  }
}

struct Tweet {
  author: String,
  tweet: String
}

impl Summary for Tweet {
  fn summarize(&self) -> String {
    format!("[{}] {}", self.author, self.tweet)
  }
}

struct Article {
  author: String,
  content: String
}

impl Summary for Article {}

fn main() {
  let t = Tweet {author: "RC".to_string(), tweet: "abc".to_string()};
  println!("{}", t.summarize());

  let a = Article {author: "RC".to_string(), content: "abc".to_string()};
  println!("{}", a.summarize());

  notify(t);
  notify_again(a);
}

fn notify(item: impl Summary) {
  println!("Breaking news! {}", item.summarize());
}

fn notify_again<T>(item: T) 
  where T: Summary
{
  println!("Breaking news! {}", item.summarize()); 
}