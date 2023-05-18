/*
Type Annotations
=====================

° Required for function signature (name)
° Types are usually inferred by the Rust compiler
° Can also be specified in code
    -  called : Explicit type annotation

*/

/* examples
----------- */

fn print_many(msg: &str, count: i32) {}

enum Mouse {
    LeftClick,
    RightClick,
    MiddleClick,
}

fn main() {
    let num = 15;
    let a = 'A';
    let left_click = Mouse::LeftClick;

    // Note: to store a vector in a structure, you must provide its type annotation
    let numbers: Vec<i32> = vec![1, 2, 3];
    let numbers_2: Vec<char> = vec!['a', 'b', 'c'];
    let mouseClicks: Vec<Mouse> = vec![Mouse::LeftClick, Mouse::LeftClick, Mouse::RightClick];
}
