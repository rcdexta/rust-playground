struct AveragedCollection {
  vector: Vec<i32>,
  average: f64
}

impl AveragedCollection {
  fn new() -> AveragedCollection {
    AveragedCollection {vector: vec![], average: 0.0}
  }

  fn add(&mut self, value: i32) {
    self.vector.push(value);
    self._update_average();
  }

  fn average(&self) -> f64 {
    self.average
  }

  fn _update_average(&mut self) {
    let sum: i32 = self.vector.iter().sum();
    self.average = sum as f64 / self.vector.len() as f64;
  }
}

fn main(){
  let mut ac = AveragedCollection::new();

  ac.add(4);
  ac.add(10);
  ac.add(1);
  ac.add(14);

  println!("Average: {}", ac.average());
}