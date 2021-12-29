struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn set_width(&mut self, w: u32) {
        self.width = w;
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width >= other.width && self.height >= other.height
    }
}

struct Color(u8, u8, u8, f32);
struct Point(f32, f32, f32);

fn main() {
    let mut admin = User {
        email: String::from("admin@company.com"),
        username: String::from("admin"),
        active: true,
        sign_in_count: 1,
    };

    admin.email.push_str("+test");

    let user2 = User {
        email: String::from("someone@email.com"),
        ..admin
    };

    let user3 = build_user(String::from("Kyle69@asdf.com"), String::from("kyle"));

    println!("{} {} {}", admin.email, user2.email, user3.email);

    let africa = Point(0.134, 55.34, 9.14);
    let fuscia = Color(128, 34, 192, 0.2);

    println!(
        "{} {} {} and {} {} {} {}",
        africa.0, africa.1, africa.2, fuscia.0, fuscia.1, fuscia.2, fuscia.3,
    );

    let mut rect = Rectangle {
        width: 30,
        height: 50,
    };
    
    let smaller = Rectangle {
        width: 10,
        height: 20,
    };

    println!(
        "Area for {:#?} (or {:?}) is {} px^2",
        rect,
        rect,
        rect.area(),
        
    );

    rect.set_width(23);

    println!(
        "Area for {:#?} (or {:?}) is {} px^2",
        rect,
        rect,
        rect.area()
    );

}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
