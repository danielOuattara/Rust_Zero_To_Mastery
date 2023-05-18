fn main() {
    let greetings = say_hello("Daniel");
    println!("{:?}", greetings);
    //-------------------------------------------

    let x = add(1, 1);
    println!("x  =  {}", x)
    // println! {"x = {}", add(1,2)}

    //-------------------------------------------

    // println!("{}", x);
    // let y = add(3, 0);
    // let z = add(x, 1);
}

fn say_hello(name: &str) -> str {
    "Hello ".to_owned() + name;
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}
