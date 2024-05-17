fn main() {
    let immutable = "rust";
    // If you wan to mutate the above variable, you have to add `mut` keyboard

    {
        let immutable = "mutated-scoped";
        // If you redeclare the variable again, it will now be shadowed

        println!("{}", immutable);
    }
    let immutable = "mutated";
    // If you redeclare the variable again, it will now be shadowed

    println!("{}", immutable);
}
