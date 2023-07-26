/*

=== VECTOR ===

A vector is a data structure.

° Multiple pieces of data
    - must be the same type

° Used for lists of information

° Can add, remove and traverse the entries

*/

fn main() {
    //option 1 : all in one
    let my_numbers = vec![1, 2, 3];
    println!("{:?}", my_numbers);

    println!("------------------------------");
    // option2 : new Vector then add one by one
    let mut my_numbers = Vec::new();

    my_numbers.push(9);
    my_numbers.push(8);

    my_numbers.push(7);
    my_numbers.pop();

    println!("{:?}", my_numbers);
    println!("my_numbers.len() = {}", my_numbers.len());

    println!("------------------------------");

    let eight = my_numbers[1];
    println!("{:?}", eight);

    println!("------------------------------");

    let _my_numbers_2 = Vec::<i32>::new();

    // iterating in a vec
    let my_numbers = vec![1, 2, 3];

    for number in my_numbers {
        println!("{:?}", number);
    }
}
