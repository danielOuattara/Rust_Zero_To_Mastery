/*

=== VECTOR ===

° Multiple pieces of data
    - must be the same type

° Used for lists of information

° Can add, remove and traverse the entries

*/

fn main() {
    //option 1
    let my_numbers = vec![1, 2, 3];
    println!("{:?}", my_numbers);

    // option2
    let mut my_numbers = Vec::new();
    my_numbers.push(1);
    my_numbers.push(2);
    my_numbers.push(3);
    my_numbers.pop();
    my_numbers.len(); // => 2
    println!("{:?}", my_numbers);

    let two = my_numbers[1];
    println!("{:?}", two);

    let _my_numbers_2 = Vec::<i32>::new();

    // iterating in a vec
    let my_numbers = vec![1, 2, 3];
    for number in my_numbers {
        println!("{:?}", number);
    }

    //==============================================  Demo start
    let my_scores = vec![
        Test { score: 90 },
        Test { score: 88 },
        Test { score: 77 },
        Test { score: 93 },
    ];

    for test in my_scores {
        println!("{:?}", test.score);
    }
}

struct Test {
    score: i32,
}
// A struct for the demo
