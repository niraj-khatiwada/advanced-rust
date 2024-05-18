fn main() {
    let mut msg = String::from("Hello");

    change_msg(&mut msg);

    println!("Message = {msg}");
}

fn change_msg(msg: &mut String) {
    *msg = String::from("Bye");
}
