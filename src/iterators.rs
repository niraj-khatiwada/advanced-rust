use authentication::Person;

fn main() {
    let people: [Person; 2] = [Person::new("Niraj"), Person::new("Niraj K")];

    let searched_person = people.iter().find(|person| person.get_name() == "NirajK");

    println!("Found {:?}", searched_person.ok_or(Person::new("Niraj K")));

    let people_names: Vec<String> = people.iter().map(|person| person.get_name()).collect();
    println!("Names = {:?}", people_names);

    for person in &people {
        println!("{:?}", person)
    }

    let mut people_vec: Vec<Person> = people.to_vec();
    people_vec.insert(1, Person::new("NK"));
    println!("{:?}", people_vec);
    let people_iter = people_vec.into_iter();
    println!(
        "{:?}",
        people_iter.filter(|person| !person.get_name().is_empty())
    );

    let one_to_hundred: Vec<u8> = (0..=100).collect();
}
