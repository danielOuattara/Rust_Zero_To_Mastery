fn main() {
    let some_bool = true;
    match some_bool {
        true => println!("it true"),
        false => println!("it's false"),
    }

    match some_bool {
        true => println!("it true"),
        _ => println!("it's false"),
    }

    let some_int = 3;
    match some_int {
        1 => println!("its 1"),
        2 => println!("its 2"),
        3 => println!("its 3"),
        _ => println!("it is something else"),
    }

    // Demo below

    let my_name = "John";

    match my_name {
        "John" => println!("That is my name"),
        "Bob" => println!("Hello Bob !"),
        "Alice" => println!("Hello Alice !"),
        _ => println!("Nice to meet you !"),
    }
}
