/*

A derive macro: to automatically implement functionality
                to struct & enum
*/

//----------------------------------------- No derive macro
/*

enum Position {
    Manager,
    Supervisor,
    Worker,
}

struct Employee {
    position: Position,
    work_hours: u8,
}

fn main() {
    let me = Employee {
        position: Position::Worker,
        work_hours: 40,
    };

    match me.position {
        Position::Manager => println!("Manager"),
        Position::Supervisor => println!("Supervisor"),
        Position::Worker => println!("Worker"),
    }
} */

//----------------------------------------- Using derive

// "derive" is a special macro applied for enums and structs
// to implement a special functionality, here is Debug functionality

// #[derive(Debug)] // to implement debug-printing functionality
// enum Position {
//     Manager,
//     Supervisor,
//     Worker,
// }

// #[derive(Debug)] // to implement debug-printing functionality
// struct Employee {
//     position: Position,
//     work_hours: u8,
// }

// fn main() {
//     let me = Employee {
//         position: Position::Worker,
//         work_hours: 40,
//     };

//     println!("{:?}", me.position);

//     println!(" me = {:?}", me);
// }

//----------------------------------------------- Using Debug, Clone, Copy

#[derive(Debug, Clone, Copy)]
enum Position {
    Manager,
    Supervisor,
    Worker,
}

#[derive(Debug, Clone, Copy)]
struct Employee {
    position: Position,
    work_hours: u8,
}

// NOTE: No 'borrow' symbol (&) on Employee, because of Clone and Copy
fn print_employee(employee: Employee) {
    println!("{:?}", employee)
}

fn main() {
    let me = Employee {
        position: Position::Worker,
        work_hours: 40,
    };

    println!("{:?}", me.position);
    println!(" me = {:?}", me);

    print_employee(me);
    print_employee(me);
}
