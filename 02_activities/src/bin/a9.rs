// Topic: Data management using tuples
//
// Requirements:
// * Print whether the y-value of a cartesian coordinate is
//   greater than 5, less than 5, or equal to 5
//
// Notes:
// * Use a function that returns a tuple
// * Destructure the return value into two variables
// * Use an if..else if..else block to determine what to print

fn coordinator() -> (i32, i32) {
    return (1, 5);
}
fn main() {
    let (_x, _y) = coordinator();
    if _y > 5 {
        println!("{} greater than 5 ", _y)
    } else if _y < 5 {
        println!("{} less than 5 ", _y)
    } else {
        println!("{} is equal to 5 ", _y)
    }
}
