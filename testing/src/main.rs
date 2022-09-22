use std::time::Instant;
use std::sync::Arc;
use std::rc::Rc;

#[derive(Clone)]
struct User {
    _is_bot: bool,
    _name: String,
    _avatar: Option<String>,
}

impl User {
    pub fn new() -> Self {
        Self {
            _is_bot: true,
            _name: "CircuitSacul".to_string(),
            _avatar: Some("skldfja;dlfkjas;dklfj;adsljfa;kldfja;".to_string()),
        }
    }
}

fn main() {
    const N: i64 = 100_000_000;
    {
        let _user = User::new();

        let now = Instant::now();
        for _ in 0..N {
            let _user = _user.clone();
        }
        let elapsed = now.elapsed();
        println!("Elapsed: {:.2?}", elapsed);
    }

    {
        let _user = Arc::new(User::new());

        let now = Instant::now();
        for _ in 0..N {
            let _user = _user.clone();
        }
        let elapsed = now.elapsed();
        println!("Elapsed: {:.2?}", elapsed); 
    }

    {
        let _user = Rc::new(User::new());

        let now = Instant::now();
        for _ in 0..N {
            let _user = _user.clone();
        }
        let elapsed = now.elapsed();
        println!("Elapsed: {:.2?}", elapsed); 
    }
}