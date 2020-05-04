fn main() {
    let ip1 = IPAddress {
        version: IpAddressVersion::IP4,
        address: String::from("127.0.0.1"),
    };
    let ip2 = IPAddress {
        version: IpAddressVersion::IP6,
        address: String::from("2001:db8::1:0:0:1"),
    };

    println!("{:?}", ip1);
    println!("{:?}", ip2);

    let loopback = IPAddressKind::V6(String::from("::1"));
    println!("{:?}", loopback);

    let home = IPAddressKind::V4(127, 0, 0, 1);
    println!("{:?}", home);

    println!("{}", find_value(Coin::Penny));
    println!("{}", find_value(Coin::Nickel));
    println!("{}", find_value(Coin::Dime));
    println!("{}", find_value(Coin::Quarter));

    let six = plus_one(Some(5));
    println!("{:?}", six);

    let none = plus_one(None);
    println!("{:?}", none);

    println!("{}", check_quarter(Coin::Dime));
    println!("{}", check_quarter(Coin::Quarter));
}

#[derive(Debug)]
enum IpAddressVersion {
    IP4,
    IP6,
}
#[derive(Debug)]
struct IPAddress {
    version: IpAddressVersion,
    address: String,
}

#[derive(Debug)]
enum IPAddressKind {
    V4(u32, u32, u32, u32),
    V6(String),
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn find_value(coin: Coin) -> i32 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,        
        Coin::Quarter => 25,
        _ => -1
    }
}

fn check_quarter(coin: Coin) -> i32 {
  if let Coin::Quarter = coin {
    25
  } else {
    0
  }
}


fn plus_one(x: Option<i32>) -> Option<i32> {
  match x {
    None => None,
    Some(i) => Some(i+1)
  }
}