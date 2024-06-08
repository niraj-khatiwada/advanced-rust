// use std::{sync::Mutex, thread};

// static SHARED_DATA: Mutex<u32> = Mutex::new(0);

// fn main() {
//     let handler = thread::spawn(poisoner);
//     handler.join().unwrap();

//     let lock = SHARED_DATA.lock().unwrap();
//     println!("Value is {}", lock);
// }

// // Here, the block panicked when it was holding a exclusive lock. This will result unexpected results so it has poisoned the mutex.
// // This can be avoided, if we use `parking_lot` crate. In `parking_lot` crate, there's no such concept of lock.unwrap() and poisoning. So the mutexes data won't be corrupted.
// fn poisoner() {
//     let mut lock = SHARED_DATA.lock().unwrap();
//     *lock += 1;
//     panic!()
// }

use std::thread;

use parking_lot::Mutex;

static SHARED_DATA: Mutex<u32> = Mutex::new(0);

fn main() {
    let handler = thread::spawn(poisoner);
    handler.join().unwrap();

    let lock = SHARED_DATA.lock();
    println!("Value is {}", lock);

    // This code will still panic but the data hold my mutex won't be corrupted.
}

fn poisoner() {
    let mut lock = SHARED_DATA.lock(); // No unwrap() here
    *lock += 1;
    panic!()
}
