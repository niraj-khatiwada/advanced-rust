fn main() {
    let mut name = "Niraj";

    let new_name = {
        name = "Programmer";
        name
    };

    println!("Name = {name:?}");
    println!("New Name = {new_name:?}");
}
