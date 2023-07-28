struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username, // field init shorthand
        email,
        sign_in_count: 1,
    }
}

fn print_user(user: User) {
    println!(
        "user: {} {} {} {}",
        user.active, user.username, user.email, user.sign_in_count
    );
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

struct AlwaysEqual;

fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("user1"),
        email: String::from("user1@user.com"),
        sign_in_count: 1,
    };

    user1.email = String::from("user.one@user.com");

    let user2 = build_user(String::from("user2@email.com"), String::from("user2"));

    let user3 = User {
        email: String::from("user3@email.com"),
        username: String::from("user3"),
        ..user2
    };

    print_user(user1);
    print_user(user2);
    print_user(user3);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    let subject = AlwaysEqual;
    
}
