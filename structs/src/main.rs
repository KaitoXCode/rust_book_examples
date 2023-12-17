struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

// tuple struct == namedtuple from python?
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

// unit-like structs
struct AlwaysEqual;

fn main() {
    // init struct
    let mut user1 = build_user(
        String::from("someone@example.com"),
        String::from("someusername123"),
    );
    println!("user1 active: {}", user1.active);
    println!("user1 username: {}", user1.username);
    println!("user1 email: {}", user1.email);
    println!("user1 sign_in_count: {}", user1.sign_in_count);
    user1.email = String::from("anotheremail@example.com");
    println!("user1 email: {}", user1.email);

    // create new user from existing user
    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };
    println!("user2 active: {}", user2.active);
    println!("user2 username: {}", user2.username);
    println!("user2 email: {}", user2.email);
    println!("user2 sign_in_count: {}", user2.sign_in_count);

    // Using Tuple Structs Without Named Fields to Create Different Types
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    println!("black: {}", black.0);
    println!("origin: {}", origin.0);

    // unit-like structs without any fields
    let _subject = AlwaysEqual;
}
