use std::thread;

use crossbeam::channel::{self, Receiver, Sender};

fn main() {
    let (main_tx, main_rx): (Sender<String>, Receiver<String>) = channel::unbounded();
    let (child_tx, child_rx): (Sender<String>, Receiver<String>) = channel::unbounded();

    let handler = thread::spawn(move || loop {
        if let Ok(msg) = child_rx.recv() {
            println!("Child received message = {}", msg);
            main_tx.send(String::from("COMPLETED")).unwrap();
            if msg == "STOP" {
                println!("Stopping now...");
                break;
            }
        }
    });

    child_tx.send(String::from("START")).unwrap();

    let msg = main_rx.recv().unwrap();
    println!("Main received msg = {}", msg);

    child_tx.send(String::from("STOP")).unwrap();

    handler.join().unwrap();
}
