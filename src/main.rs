fn main() {
    let v = vec![5, 3, 8, 9, 10, 19, 12, -1];

    let largest = find_largest(&v);

    println!("Largest number: {}", largest);

    let a = Point { x: 1, y: 2 };
    let b = Point { x: 1.0, y: 2.0 };
    println!("{:?} {:?}", a, b);
}

fn find_largest<T>(list: &[T]) -> T
where
    T: PartialOrd + Copy,
{
    let mut largest = list[0];

    for &item in list {
        if largest < item {
            largest = item;
        }
    }

    largest
}

#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}
