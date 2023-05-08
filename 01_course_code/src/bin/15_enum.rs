// enum Direction {
//     Up,
//     Down,
// }

// fn main() {
//     let position = Direction::Up;
//     match position {
//         Direction::Up => println!("Go up"),
//         Direction::Down => println!("Go down"),
//     };
// }

//----------------------------------------------------

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn where_to(go: Direction) {
    match go {
        Direction::Up => println!("Go up"),
        Direction::Down => println!("Go down"),
        Direction::Left => println!("Go left"),
        Direction::Right => println!("Go right"),
    };
}

fn main() {
    where_to(Direction::Up);
}
