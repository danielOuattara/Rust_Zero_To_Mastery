fn main() {
    let mut i = 0;
    loop {
        if i == 5 {
            break;
        }
        println!("{:?}", i);
        i += 1;
    }
}
