use rand::Rng;

fn main() {
    println!("{:?}", get_rand());
}

fn get_rand() -> u8 {
    let mut thread_rng = rand::thread_rng();
    let random: bool = thread_rng.gen();

    // If you want to return a value from a function inside a block, you must use explicit return statement.
    // If you just use 2 or 3 like shown below, Rust understands it as the return value of the scope not the return value of the function.
    // So make sure to use explicit return inside a block for returning value from a function.
    let val = if random { 2 } else { 3 };
    // let val = if random { return 2 } else { return 3 }; -> This will return the value from inside the scope. The line below this will never be executed.

    println!("{val}");

    10
}
