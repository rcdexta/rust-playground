struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    calculation: T,
    value: Option<u32>,
}

impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}

fn main() {
    // closure to print
    let print = |str: &str| println!("{}", str);

    let add_one = |x| x + 1;

    let s = "hello";
    print(s);

    let x = "hello".to_string();
    print(&x);

    println!("{}", add_one(5));

    let mut cacher = Cacher::new(|x| x + 1);

    println!("{:?}", cacher.value(10));
}
