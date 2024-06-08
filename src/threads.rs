use std::thread::{self, JoinHandle, ScopedJoinHandle};

fn main() {
    let mut thread_handlers: Vec<thread::JoinHandle<()>> = Vec::new();
    thread_handlers.push(thread::spawn(this_should_work_in_thread));
    thread_handlers.push(
        thread::Builder::new()
            .name(String::from("custom-thread"))
            .spawn(move || {
                for i in 1..=10 {
                    println!("{i}")
                }
                println!("Current thread name= {:?}", thread::current().name());
                // By default, spawned threads do not have name. You have to use thread Builder to assign it a name.
            })
            .unwrap(),
    );
    thread_handlers.into_iter().for_each(|handler| {
        handler.join().unwrap();
    });

    println!("Current thread name= {:?}", thread::current().name());

    // Normal Threads
    let list: Vec<u8> = (0..=100).collect();

    let handler: JoinHandle<Vec<u8>> = thread::spawn(move || {
        return list.iter().map(|el| el + 1).collect();
    });

    // println!("{:?}", list); // You cannot access it after it's moved
    println!("{:?}", handler.join().unwrap());

    // Scoped Threads
    // Basically, the scope thread will only continue when all other threads are done performing when it reaches this part
    let list_scoped: Vec<u8> = (0..=100).collect();
    let scoped_list = thread::scope(|s| {
        let scoped_handler: ScopedJoinHandle<Vec<u8>> =
            s.spawn(|| list_scoped.iter().map(|el| el + 1).collect());
        scoped_handler.join().unwrap()
    });
    println!("?? {:?}", list_scoped);
    // ⬆️ This part will only run when the above scoped thread is done performing the task.
    // It basically makes the normal thread deterministic.
    // You can still use the variables that was used inside a scoped threads(See that `move` was not applied in spawned scoped threads).
    println!("Scoped List{:?}", scoped_list)
}

fn this_should_work_in_thread() {
    println!("Inside a worker")
}
