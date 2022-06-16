#[derive(Debug)]
struct User {
    username: String,
    email: Option<String>,
    sign_in_count: u64,
}

impl User {
    fn new(username: &str, email: Option<&str>) -> User {
        let username = username.to_owned();
        let email: Option<String> = match email {
            None => None,
            Some(email) => Some(email.to_owned()),
        };
        return User {
            username,
            email,
            sign_in_count: 0,
        };
    }
}

fn main() {
    let user = User::new("Circuit", Some("email@example.com"));
    println!("Hello, {:?}", user);
}
