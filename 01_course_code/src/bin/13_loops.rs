fn main() {
    count_one();
    println!("-----------");
    count_up();
    println!("-----------");
    count_down();
}

fn count_one() {
    loop {
        println!("Hello");
        break;
    }
}

fn count_up() {
    let mut counter_i = 0;
    loop {
        println!("{:?}", counter_i);
        if counter_i == 10 {
            break;
        }
        counter_i += 1;
    }
}

fn count_down() {
    let mut counter_j = 5;
    loop {
        println!("{:?}", counter_j);
        counter_j -= 1;
        if counter_j == 0 {
            break;
        }
    }
    println!("Done !");
}
