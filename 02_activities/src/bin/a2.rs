// Topic: Basic arithmetic
//
// Program requirements:
// * Displays the result of the sum of two numbers
//
// Notes:
// * Use a function to add two numbers together
// * Use a function to display the result
// * Use the "{:?}" token in the println macro to display the result

fn adder(number1: i32, number2: i32) -> i32 {
    number1 - number2
}

fn result_displayer(result: i32) {
    println!("{:?}", result);
}

fn main() {
    let result = adder(3, 4);
    result_displayer(result);
}
