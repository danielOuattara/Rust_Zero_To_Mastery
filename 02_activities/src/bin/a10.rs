// Topic: Working with expressions
//
// Requirements:
// * Print "its big" if a variable is > 100
// * Print "its small" if a variable is <= 100
//
// Notes:
// * Use a boolean variable set to the result of
//   an if..else expression to store whether the value
//   is > 100 or <= 100
// * Use a function to print the messages
// * Use a match expression to determine which message
//   to print

fn message_printer(arg: bool) {
    match arg {
        true => println!("its big"),
        false => println!("its small"),
    };
}

fn main() {
    let _number = 90;
    // let _is_greater_than_100 = if _number > 100 { true } else { false };
    let _is_greater_than_100 = _number > 100;

    message_printer(_is_greater_than_100)
}
