/*

=== STRINGS  ===
° Two commonly used types of String
    - String type = owned type
    - &str = borrowed String slice

° Must use an owned string (String) to store data in a struct

° Use &str when passing to a function

 */

//====================================================================

// Example 1 : Pass string data to a function

// fn print_it(data: &str) {
//     // to use a string in that function, that string must be borrowed (&)
//     println!("{:?}", data)
// }

// fn main() {
//     //----------------------------- example 1
//     print_it("A string slice");

//     let owned_string = "Owned string".to_owned();
//     let another_owned_string = String::from("Another owned string");

//     println!("{}", owned_string);
//     println!("{}", another_owned_string);

//     print_it(&owned_string);
//     print_it(&another_owned_string);
// }

//=====================================================================

// Example 2 : struct not working first
// --------------------------------------
// struct Employee {
//     name: &str,
// }

// Example 3 : struct working
//---------------------------

struct Employee {
    name: String,
}

fn main() {
    // //------------------------------ example 2
    // let employee_name = "Jayson";
    // let employee = Employee {
    //     name: employee_name,
    // };

    //------------------------------ example 3
    let _employee_1_name = "Jayson".to_owned();
    let _employee_2_name = String::from("Marvel");

    let employee_1 = Employee {
        name: "Jayson".to_owned(),
    };
    println!("{:?}", employee_1.name);

    let employee_2 = Employee {
        name: String::from("Marvel"),
    };
    println!("{:?}", employee_2.name);
}
