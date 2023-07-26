/*

=== VECTOR ===

A vector is a data structure.

° Multiple pieces of data
    - must be the same type

° Used for lists of information

° Can add, remove and traverse the entries

*/

struct Test {
    score: i32,
}
// A struct for the demo

fn main() {
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
