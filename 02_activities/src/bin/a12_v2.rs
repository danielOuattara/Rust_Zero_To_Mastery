// Topic: Implementing functionality with the impl keyword
//
// Requirements:
// * Print the characteristics of a shipping box
// * Must include dimensions, weight, and color
//
// Notes:
// * Use a struct to encapsulate the box characteristics
// * Use an enum for the box color
// * Implement functionality on the box struct to create a new box
// * Implement functionality on the box struct to print the characteristics

//-----------------------
enum Color {
    Brown,
    Orange,
    Black,
}

// impl Color {
//     fn print(&self) -> String {
//         match self {
//             Color::Brown => {
//                 return "brown".to_owned();
//             }
//             Color::Orange => {
//                 return String::from("orange");
//             }
//             Color::Black => {
//                 return "black".to_owned();
//             }
//         }
//     }
// }

impl Color {
    fn print(&self) {
        match self {
            Color::Brown => println!("Color: brown"),
            Color::Orange => println!("Color: orange"),
            Color::Black => println!("Color: black"),
        }
    }
}

struct ShippingBox {
    length: f32,
    height: f32,
    depth: f32,
    weight: f32,
    color: Color,
}

impl ShippingBox {
    fn new(length: f32, height: f32, depth: f32, weight: f32, color: Color) -> Self {
        Self {
            length,
            height,
            depth,
            color,
            weight,
        }
    }

    fn print_info(&self) {
        println!(
            "Dimensions: length = {:?}, height = {:?}, depth = {:?}",
            self.length, self.height, self.depth,
        );
        println!("Weight = {:?}", self.weight,);
        self.color.print();
        println!("--------------------------------")
    }
}

fn main() {
    let box_1 = ShippingBox::new(20.0, 15.0, 8.0, 40.1, Color::Brown);
    box_1.print_info();

    let box_2 = ShippingBox::new(10.0, 7.5, 4.0, 80.1, Color::Orange);
    box_2.print_info();

    let box_3 = ShippingBox::new(18.0, 11.0, 6.0, 35.1, Color::Black);
    box_3.print_info();
}
