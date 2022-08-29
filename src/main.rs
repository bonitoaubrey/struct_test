#[derive(Debug)]

struct Person {
    name: String,
    surname: String,
    age: u8,
    balance: f32,
}

fn main() {
    let person1 = Person {
        name: String::from("Kate"),
        surname: String::from("White"),
        age: 33,
        balance: 3000.00,
    };

    println!("{:#?}", person1);
}
