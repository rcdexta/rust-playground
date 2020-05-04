fn main() {
    let mut v1 = Vec::new();
    let v2 = vec![1, 2, 3, 4];
    
    println!("{:?}", v2);

    v1.push('A');
    v1.push('B');
    v1.push('C');
    println!("{:?}", v1);

    let e = &v2[2];
    println!("{}", e);

    match v1.get(2) {
        None => println!("There is no third char"),
        Some(third) => println!("Third Char: {}", third)
    }

    println!("All elements in v2:");
    for i in &v2 {
        println!("{}", i);
    }

    
}
