fn main() {
    while_loop();
    for_loop();
    unconditional_loop();
    loop_label_and_result();
    multiple_loops();
    range();

    // some notes:
    // can use labels for all loop types.
    // can only break with value to `loop`.
}

fn while_loop() {
    let mut counter: u32 = 0;
    while counter < 5 {
        println!("Counter is {}", counter);
        counter = counter + 1;
    }
}

fn for_loop() {
    let arr = [0, 1, 2, 3, 4];
    for value in arr {
        println!("Value is {}", value);
    }
}

fn unconditional_loop() {
    let mut counter: u32 = 0;
    loop {
        println!("Counter is {}", counter);
        counter = counter + 1;
        if counter == 5 {
            break;
        };
    }
}

fn loop_label_and_result() {
    let result = 'label: loop {
        let mut counter: u32 = 0;
        loop {
            counter = counter + 1;
            if counter == 5 {
                break 'label counter * 2;
            };
        }
    };
    println!("Broke out of outermost loop with {}", result);
}

fn multiple_loops() {
    let result = 'label: loop {
        let my_arr = [0, 1, 2, 3, 4];
        for val in my_arr {
            if val == 3 {
                break 'label val;
            };
        }
    };
    println!("Broke outer loop with {}", result)
}

fn range() {
    for val in 0..10 {
        println!("{}", val);
    }
}
