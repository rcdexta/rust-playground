fn main() {
    let user = build_user(String::from("JamesBond"));
    println!("{}", user.email);
    println!("{}", user.name);
    println!("{}", user.active);

    let another_user = build_user_from(build_user(String::from("Dalda")));
    println!("{:?}", another_user);
    println!("User active: {}", another_user.is_active());

    let rect = Rectangle {height: 3, width: 5};
    println!("Area: {}", rect.area());
    let other = Rectangle {height: 2, width: 3};
    println!("Rect can contain other: {}", rect.can_hold(&other));

    let square = Rectangle::square(40);
    println!("{:?}", square);
}

#[derive(Debug)]
struct User {
    name: String,
    email: String,
    active: bool
}

impl User {
    fn is_active(&self) -> bool {
        self.active
    }
}

fn build_user(name: String) -> User {
    User {
        name,
        email: String::from("jb@007.com"),
        active: true,
    }
}

fn build_user_from(user: User) -> User {
    User {        
        email: String::from("jb@007.com"),
        active: true,
        ..user
    }
}


#[derive(Debug)]
struct Rectangle {
    height: u32,
    width: u32
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.height * self.width
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(side: u32) -> Rectangle {
        Rectangle {height: side, width : side}
    }
}