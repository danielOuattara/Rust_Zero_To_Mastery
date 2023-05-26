// Topic: Option
//
// Requirements:
// * Print out the details of a student's locker assignment
// * Lockers use numbers and are optional for students
//
// Notes:
// * Use a struct containing the student's name and locker assignment
// * The locker assignment should use an Option<i32>

struct Student {
    name: String,
    locker: Option<i32>,
}

fn main() {
    let john = Student {
        name: "John".to_owned(),
        locker: None,
    };

    println!("student : {:?}", john.name);
    match john.locker {
        Some(number) => println!("locker is : {:?}", number),
        None => println!("No locker found !"),
    }

    let marie = Student {
        name: "Marie".to_owned(),
        locker: Some(123),
    };
    println!("student : {:?}", marie.name);
    match marie.locker {
        Some(number) => println!("locker is : {:?}", number),
        None => println!("No locker found !"),
    }
}
