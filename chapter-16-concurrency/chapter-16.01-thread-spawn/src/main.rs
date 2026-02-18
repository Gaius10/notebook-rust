use std::thread;
use std::time::Duration;

fn main() {
    // Spawn a new thread and move the closure into it.
    // Notice that the main thread continues executing while the spawned thread is running.

    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {i} from the spawned thread!");
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {i} from the main thread!");
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();


    // Moving ownership to the thread
    let v = vec![1, 2, 3];

    // As rust compiler can't know how long v will live, it's not
    // possible to borrow it to the closure, it's necessary to move it.
    let handle = thread::spawn(move || {
        println!("Here's a vector: {v:?}");
    });

    handle.join().unwrap();
}
