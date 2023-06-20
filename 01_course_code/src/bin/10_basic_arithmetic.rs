fn subtractor(number1: i32, number2: i32) -> i32 {
    number1 - number2
}

fn main() {
    let summation = 2 + 3;
    println!("2 + 3 = {}", summation);

    let subtraction = 10 - 5;
    println!("10 - 5 = {}", subtraction);

    let division = 10 / 2;
    println!("10 / 2 = {}", division);

    let multiplication = 5 * 10;
    println!("5 * 10 = {}", multiplication);

    let sub_result = subtractor(8, 3);

    println!("sub_result = {}", sub_result);

    let modulo = 20 % 3;
    println!("20 % 3 = {} ", modulo);
}

/* stopped @ 58" */
