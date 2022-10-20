struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
/*
 * In the User struct definition in Listing 5-1, we used the owned String type rather than the
 * &str string slice type. This is a deliberate choice because we want each instance of this struct
 * to own all of its data and for that data to be valid for as long as the entire struct is valid.
 */
fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someuserName123"),
        active: true,
        sign_in_count: 1,
    };

    let user2 = User {
        email: String::from("someanotherEmail@example.com"),
        ..user1
    };

    user1.email = String::from("anotherUser@example.com");

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}
