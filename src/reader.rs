use std::io::{BufRead, BufReader};

fn main() {
    let mut buf: String = String::from("");

    println!("What's your name?");
    let mut buf_reader = BufReader::new(std::io::stdin());

    if std::io::BufReader::read_line(&mut buf_reader, &mut buf).is_ok() {
        println!("Hello {}", buf.trim());
    }
}
