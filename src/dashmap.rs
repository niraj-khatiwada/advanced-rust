// DashMap = HashMap but for concurrency on multi-threaded apps

use core::time;
use dashmap::DashMap;
use once_cell::sync::Lazy;
use std::thread;

static SHARED_DATA: Lazy<DashMap<u8, u8>> = Lazy::new(DashMap::new);

fn main() {
    for i in 1..=100 {
        thread::spawn(move || {
            if i % 2 == 0 {
                SHARED_DATA.insert(i, i);
            } else if let Some(mut entry) = SHARED_DATA.get_mut(&(i - 1)) {
                *entry += 1;
            }
        });
    }
    thread::sleep(time::Duration::from_secs(2));
    println!("{:?}", SHARED_DATA);
    // We'll only have data for even.
    if let Some(entry) = SHARED_DATA.get_mut(&2) {
        println!("{:?}", *entry);
    }
}
