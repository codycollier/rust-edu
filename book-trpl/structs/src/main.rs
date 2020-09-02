fn main() {
    #[derive(Debug)]
    struct User {
        username: String,
        email: String,
        sign_in_count: u64,
        active: bool,
    };

    let user = User {
        username: String::from("beatrice"),
        email: String::from("b@somewhere.com"),
        sign_in_count: 1,
        active: true,
    };
    println!("user: {:?}", user);
}
