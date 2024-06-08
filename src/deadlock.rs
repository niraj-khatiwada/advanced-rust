use once_cell::sync::Lazy;
use std::sync::{Mutex, RwLock};

static shared_data: Mutex<u32> = Mutex::new(0);
fn main() {
    // // DeadLock
    // let lock = shared_data.lock().unwrap(); // There's no chance that this drop will ever be released in this case. Hence, deadlock
    // let lock = shared_data.lock().unwrap(); // -> We tried to lock the mutex when the previous lock was not released.

    // Solution
    // {
    //     let lock = shared_data.lock().unwrap();
    //     // A mutex lock is unlocked when it reaches to the end of scopes
    // }
    // let lock = shared_data.lock().unwrap();

    // You can manually release the lock as well
    // let lock = shared_data.lock().unwrap();
    // std::mem::drop(lock);
    // let lock = shared_data.lock().unwrap();

    // Same goes for read-write locks as well.
    // let lazy_shared_data: Lazy<RwLock<u32>> = Lazy::new(|| RwLock::new(0));
    // let read_lock = lazy_shared_data.read().unwrap();
    // let write_lock = lazy_shared_data.write().unwrap();

    // let lazy_shared_data: Lazy<RwLock<u32>> = Lazy::new(|| RwLock::new(0));
    // {
    //     let read_lock = lazy_shared_data.read().unwrap();
    // }
    // let write_lock = lazy_shared_data.write().unwrap();

    // Use try_lock instead. This will not wait for another lock to be released.
    let lock = shared_data.lock().unwrap();
    if shared_data.try_lock().is_ok() {
        println!("Lock acquired.");
    } else {
        println!("Cannot acquire lock.");
    }
}
