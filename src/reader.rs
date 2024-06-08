// use std::io::{BufRead, BufReader};

// fn main() {
//     let mut buf: String = String::from("");

//     println!("What's your name?");
//     let mut buf_reader = BufReader::new(std::io::stdin());

//     if std::io::BufReader::read_line(&mut buf_reader, &mut buf).is_ok() {
//         println!("Hello {}", buf.trim());
//     }
// }

// More simple way
fn main() {
    let mut buf = String::new();
    println!("What's your name?");
    std::io::stdin().read_line(&mut buf).unwrap();
    println!("Hello, {:?}", buf.trim())
}
