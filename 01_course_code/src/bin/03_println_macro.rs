fn main() {
    let life = 42;

    println!("Hello"); // ! --> to indicate a macro
    println!("{}", life); // { } --> token + end user display vision
    println!("{:?}", life); // :? --> to indicate a debug display vision
    println!("{:?} {}", life, life);
    println!("The meaning is {:?}", life);

    // new !

    println!("{life:?}");
    println!("{life}");
    println!("The meaning is {life}");
    println!("The meaning is {life:?}");

    /*

    {:?}

    {varname:?}

     */
}
