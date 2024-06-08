// // Unsafe shared ability
// use std::thread;

// static mut COUNTER: u32 = 0;

// fn main() {
//     let mut handler: Vec<thread::JoinHandle<()>> = Vec::new();
//     for _ in 0..1000 {
//         handler.push(thread::spawn(|| {
//             for _ in 0..100 {
//                 unsafe {
//                     COUNTER += 1;
//                 }
//             }
//         }));
//     }
//     handler.into_iter().for_each(|h| {
//         h.join().unwrap();
//     });

//     unsafe {
//         println!("{}", COUNTER);
//     }
// }

// // Safe operation
// use std::sync::Mutex;
// use std::thread;

// static COUNTER: Mutex<u32> = Mutex::new(0);

// fn main() {
//     let mut handler: Vec<thread::JoinHandle<()>> = Vec::new();
//     for _ in 0..1000 {
//         handler.push(thread::spawn(|| {
//             for _ in 0..100 {
//                 let mut lock = COUNTER.lock().unwrap();
//                 *lock += 1; // -> Since we're only writing the data, it'se better to use write lock
//             }
//         }));
//     }
//     handler.into_iter().for_each(|h| {
//         h.join().unwrap();
//     });

//     println!("{}", COUNTER.lock().unwrap()); // -> Since we're only reading the data, it's better to use read lock.
// }

// Mutual Exclusive but with exclusive read and write
use once_cell::sync::Lazy;
use std::sync::RwLock;
use std::thread;

static COUNTER: Lazy<RwLock<u32>> = Lazy::new(|| RwLock::new(0));

fn main() {
    let mut handler: Vec<thread::JoinHandle<()>> = Vec::new();
    for _ in 0..1000 {
        handler.push(thread::spawn(|| {
            for _ in 0..100 {
                let mut lock = COUNTER.write().unwrap();
                *lock += 1;
            }
        }));
    }
    handler.into_iter().for_each(|h| {
        h.join().unwrap();
    });

    println!("{}", COUNTER.read().unwrap());
}
