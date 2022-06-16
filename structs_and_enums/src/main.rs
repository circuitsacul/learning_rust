#[derive(Debug)]
struct User {
    username: String,
    email: Option<String>,
    sign_in_count: u64,
}

enum KwargOption<T> {
    NoChange,
    Value(T),
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

    fn clone(&self, username: KwargOption<&str>, email: KwargOption<Option<&str>>) -> User {
        User::new(
            match username {
                KwargOption::NoChange => &self.username,
                KwargOption::Value(username) => username,
            },
            match email {
                KwargOption::NoChange => match &self.email {
                    None => None,
                    Some(email) => Some(email),
                },
                KwargOption::Value(email) => email,
            },
        )
    }
}

#[derive(Debug)]
enum Command {
    SendMessage(String),
    Disconnect(bool),
}

impl Command {
    fn call(&self) {
        println!("Enum called!!! {:?}", self);
    }
}

#[derive(Debug)]
enum Currency {
    Penny(usize),
    Quarter(usize),
    Dollar(usize),
}

impl Currency {
    fn in_pennies(&self) -> Currency {
        match self {
            Currency::Penny(val) => Currency::Penny(*val),
            Currency::Quarter(val) => Currency::Penny(val*25),
            Currency::Dollar(val) => Currency::Penny(val*100),
        }
    }
}

fn main() {
    let user = User::new("Circuit", Some("email@example.com"));
    println!("Hello, {:#?}", user);

    let new_user = user.clone(
        KwargOption::NoChange,
        KwargOption::Value(Some("newemail@example.com")),
    );
    println!("Cloned user into {:#?}", new_user);

    let cmd = Command::SendMessage(String::from("Hello, World!"));
    cmd.call();

    let five_dollars = Currency::Dollar(5);
    println!("I have: {:?}", five_dollars);
    let in_pennies = five_dollars.in_pennies();
    println!("That's {:?}", in_pennies);
}
