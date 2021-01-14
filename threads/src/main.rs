use std::sync::{Arc, mpsc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    /*
        Threads section
    */
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

    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    handle.join().unwrap();

    /*
        Multiple producer single consumer message passing section
    */
    let (tx0, rx) = mpsc::channel();
    let tx1 = mpsc::Sender::clone(&tx0);

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx0.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("vals"),
            String::from("for"),
            String::from("you!"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for rec in rx {
        println!("Got: {}", rec);
    }

    /*
        Shared state concurrency/mutex section
    */
    let m = Mutex::new(5);

    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }

    println!("m = {:?}", m);
    
    let ctr = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let ctr = Arc::clone(&ctr);
        let handle = thread::spawn(move || {
            let mut num = ctr.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *ctr.lock().unwrap());
}
