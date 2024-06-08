use std::collections::HashMap;

fn main() {
    let mut object: HashMap<u8, String> = HashMap::new();

    object.insert(1, String::from("Niraj"));
    object.insert(1, String::from("Niraj K"));

    println!("{:?}", object);

    println!("{:?}", object.get(&1_u8));

    for (key, value) in object.iter() {
        println!("{} > {}", key, value);
    }
}
