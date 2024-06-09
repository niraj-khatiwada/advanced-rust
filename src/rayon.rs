use rayon::prelude::*;
use std::{
    thread::{self, JoinHandle},
    time::{self, Duration},
};

/// Run the below code in release mode to see the actual benchmark. cargo run --release
fn main() {
    let mut handlers: Vec<JoinHandle<()>> = Vec::new();

    handlers.push(thread::spawn(|| {
        let now = time::Instant::now();
        let numbers: Vec<u64> = (0..=1_000_000_000).collect();

        let sum = numbers.par_iter().sum::<u64>();
        println!("Sum = {}", sum);

        println!("Time taken to find sum {:?}", now.elapsed()); // -> This takes ~1 second 🤯. JavaScript takes 6-7 seconds.
    }));

    // Only run  one at a time for actual benchmark
    // When you run both the tasks, if a par_iter() is already being used, then I guess rayon has to wait for that to be released so that it can lock the thread again. That's why running both of these here will make the benchmarking slow. You're supposed to do par_iter() one at a time since thread can only be locked one at a time.
    // handlers.push(thread::spawn(|| {
    //     let now = time::Instant::now();
    //     let num_of_primes: Vec<u32> = (0..=1_000_000)
    //         .into_par_iter()
    //         .filter(|num| is_prime(*num))
    //         .collect();
    //     println!("Found {:?} prime numbers", num_of_primes.len());
    //     println!("Time taken {:?}", now.elapsed());
    // }));

    handlers.into_par_iter().for_each(|h| h.join().unwrap());
}

fn is_prime(n: u32) -> bool {
    (2..n).into_par_iter().all(|val| n % val != 0)
}
