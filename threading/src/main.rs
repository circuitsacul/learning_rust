use std::thread;
use std::time::Duration;
use std::sync::{mpsc, Arc, Mutex};

fn main() {
    // thread basics
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();

    // moving variables into a thread
    let x = vec![1, 2, 3];
    let handle = thread::spawn(move || {
        println!("Vector {:?}", x);
    });
    handle.join().unwrap();

    // mpsc => multiple producers, single consumer
    // really epic btw
    let (tx1, rx) = mpsc::channel();
    let tx2 = tx1.clone();

    thread::spawn(move || {
        let to_send = vec![1, 2, 3];
        for item in to_send.into_iter() {
            thread::sleep(Duration::from_secs(1));
            tx1.send(item).unwrap();
        }
    });
    thread::spawn(move || {
        let to_send = vec![4, 5, 6];
        for item in to_send.into_iter() {
            thread::sleep(Duration::from_secs(1));
            tx2.send(item).unwrap();
        }
    });

    loop {
        match rx.recv() {
            Err(_) => break,
            Ok(val) => println!("Got {}", val),
        }
    }

    // sharing a vector between threads
    let list = Arc::new(Mutex::new(vec![1]));
    let mut handles = vec![];

    for x in 0..10 {
        let list = list.clone();
        let handle = thread::spawn(move || {
            let mut list_val = list.lock().unwrap();
            list_val.push(x);
        });
        handles.push(handle);
    }

    for handle in handles.into_iter() {
        handle.join().unwrap();
    }

    println!("Final list is {:?}", list);
}
