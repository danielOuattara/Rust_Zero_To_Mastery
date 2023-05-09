// Topic: Looping using the loop statement
//
// Program requirements:
// * Display "1" through "4" in the terminal
//
// Notes:
// * Use a mutable integer variable
// * Use a loop statement
// * Print the variable within the loop statement
// * Use break to exit the loop

fn main() {
    let mut counter_i = 1;

    loop {
        println!("{:?}", counter_i);
        if counter_i == 4 {
            break;
        }
        counter_i += 1;
    }
    println!("done !")
}
