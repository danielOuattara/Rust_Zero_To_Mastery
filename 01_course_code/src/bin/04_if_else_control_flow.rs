fn main() {
    let a = 99;
    //----------------------
    if a > 99 {
        println!("Big number");
    } else {
        println!("Small number");
    }

    //----------------------
    if a > 99 {
        if a > 200 {
            println!("Huge number");
        } else {
            println!("Big number");
        }
    } else {
        println!("Small number");
    }

    //----------------------
    if a > 200 {
        println!("Huge Number");
    } else if a > 99 {
        println!("Big Number")
    } else {
        println!("Small number")
    }
}
