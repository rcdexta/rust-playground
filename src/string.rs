fn main() {
    let mut s = String::from("hello");    
    print_type_of(&s);
    append_to_string(&mut s);
    println!("{}", s);

    // Structs
    let user = build_user(String::from("JamesBond"));
    println!("{}", user.email);
    println!("{}", user.name);
    println!("{}", user.active);

    let another_user = build_user_from(build_user(String::from("Dalda")));
    println!("{:?}", another_user);
    println!("User active: {}", another_user.is_active());
}

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

fn append_to_string(s: &mut String) {
    s.push_str(", worldddd!");
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