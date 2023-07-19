// Topic: Working with an enum
//
// Program requirements:
// * Prints the name of a color to the terminal
//
// Notes:
// * Use an enum with color names as variants
// * Use a function to print the color name
// * The function must use the enum as a parameter
// * Use a match expression to determine which color
//   name to print

enum Colors {
    Red,
    Green,
    Blue,
}

fn which_color(color: Colors) {
    match color {
        Colors::Red => println!("Color is Red !"),
        Colors::Green => println!("Color is Green !"),
        Colors::Blue => println!("Color is Blue !"),
    };
}
fn main() {
    which_color(Colors::Red);
    which_color(Colors::Blue);
    which_color(Colors::Green);
}
