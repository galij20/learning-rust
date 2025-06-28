#![allow(warnings)]
use std::thread;
use std::time::Duration;


fn main() {
    let v = vec![1, 2, 3];
    
    let handle = thread::spawn(move || {
        let iterate = v.iter();
        for val in iterate{
            println!("Here is the spawned threads: {val}");           
            thread::sleep(Duration::from_millis(100));
        }
    });

    for i in 1..5 {
        println!("hi number {i} from the main thread");
        thread::sleep(Duration::from_millis(100));
    }

    handle.join().unwrap();
}
