// Topic: Strings
//
// Requirements:
// * Print out the name and favorite colors of people aged 10 and under
//
// Notes:
// * Use a struct for a persons age, name, and favorite color
// * The color and name should be stored as a String
// * Create and store at least 3 people in a vector
// * Iterate through the vector using a for..in loop
// * Use an if expression to determine which person's info should be printed
// * The name and colors should be printed using a function

//-----------------------
//
// * Use a struct for a persons age, name, and favorite color
// * The color and name should be stored as a String
struct PersonData {
    age: u8,
    name: String,
    favorite_color: String,
}

// * The name and colors should be printed using a function
fn print_person_info(data1: u8, data2: &str, data3: &str) {
    println!(
        "age: {:?}, name: {:?}, favorite color: {:?}",
        data1, data2, data3
    );
}

fn main() {
    // * Create and store at least 3 people in a vector
    let person_list = vec![
        PersonData {
            age: 39,
            name: "Daniel".to_owned(),
            favorite_color: "orange".to_owned(),
        },
        PersonData {
            age: 37,
            name: String::from("Julie"),
            favorite_color: String::from("pink"),
        },
        PersonData {
            age: 5,
            name: "Gaia".to_owned(),
            favorite_color: "pink".to_owned(),
        },
        PersonData {
            age: 3,
            name: String::from("Amaya"),
            favorite_color: String::from("blue"),
        },
    ];

    // * Iterate through the vector using a for..in loop
    // * Use an if expression to determine which person's info should be printed
    for person in &person_list {
        if person.age <= 10 {
            print_person_info(person.age, &person.name, &person.favorite_color);
        }
    }
}
