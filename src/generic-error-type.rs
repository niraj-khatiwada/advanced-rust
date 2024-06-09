// anyhow trait works like this
type GenericResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() {
    println!("{:?}", can_throw_err());
    println!("{:?}", can_throw_err2());
}

fn can_throw_err() -> GenericResult<String> {
    Err(String::from("Error").into())
}

fn can_throw_err2() -> GenericResult<String> {
    Ok(String::from("No Error"))
}
