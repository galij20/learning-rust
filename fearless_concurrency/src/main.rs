use std::thread;
use std::time::Duration;


fn main() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number{i} from the spawned thread!");
            thread::sleep(Duration::from_millis(100));
        }
    });

    for i in 1..5 {
        println!("hi number {i} from the main thread");
        thread::sleep(Duration::from_millis(100));
    }

    handle.join().unwrap();
}
