struct Error {
    message: String
}

impl Error {
    fn new(message: &str) -> Self {
        Self {
            message: message.to_owned()
        }
    }
}


fn main() {
    // panic!("oops!") // unrecoverable error

    match div(10_f64, 0_f64) {
        Ok(result) => println!("10/0={}", result),
        Err(why) => println!("Failed because {}", why.message),
    };
    _ = propogate_div_error();

    // Result.exepct() will "assert" that the result is Ok,
    // otherwise panic! with the passed message.
    // Result.unwrap() is essentially the same except that
    // you don't pass your own error message.

    // also whole Box<dyn std::io::Error> thing but waiting on that
    // for stuff on traits. In any case, main() can return results.
}

fn propogate_div_error() -> Result<f64, Error> {
    // ? means, return error if it exists, otherwise
    // continue function.
    let result = div(10_f64, 0_f64)?;
    println!("result is {}", result);
    Ok(result)
}

fn div(a: f64, b: f64) -> Result<f64, Error> {
    if b == 0_f64 {
        Err(Error::new("cannot divide by 0"))
    } else {
        Ok(a / b)
    }
}
