use std::{thread, time::Duration};

async fn sleep(i: u32, time: u64) {
    println!("Timer started {}", i);
    // thread::sleep(Duration::from_millis(time)); // This'll run in sequential. This basically blocks the whole tokio thread.
    // tokio::time::sleep(Duration::from_millis(time)).await; // Solution
    let _ = tokio::task::spawn_blocking(move || thread::sleep(Duration::from_millis(time))).await; // You can use spawn blocking as well
    println!("Timer ended {}", i);
}

#[tokio::main]
async fn main() {
    tokio::join!(sleep(1, 500), sleep(2, 1000), sleep(3, 1500));
}
