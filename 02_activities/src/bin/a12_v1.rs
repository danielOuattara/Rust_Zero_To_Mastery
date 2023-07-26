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

impl Color {
    fn print(&self) {
        match self {
            Color::Brown => println!("color: brown"),
            Color::Orange => println!("color: orange"),
            Color::Black => println!("color: black"),
        }
    }
}

//-----------------------
struct BoxDimensions {
    length: f32,
    height: f32,
    width: f32,
}

impl BoxDimensions {
    fn print(&self) {
        println!(
            "dimensions: Length = {:?}, Height = {:?}, Width = {:?} ",
            self.length, self.height, self.width
        );
    }
}

//-----------------------
struct ShippingBox {
    color: Color,
    dimensions: BoxDimensions,
    weight: f32,
}

impl ShippingBox {
    fn new(color: Color, dimensions: BoxDimensions, weight: f32) -> Self {
        Self {
            color,
            dimensions,
            weight,
        }
    }

    fn print_props(&self) {
        self.dimensions.print();
        println!("weight: {:?} ", self.weight,);
        self.color.print();
    }
}

//----------------------------------

fn main() {
    let box_1_dimensions = BoxDimensions {
        length: 19.5,
        height: 10.5,
        width: 8.5,
    };

    let box_1 = ShippingBox::new(Color::Brown, box_1_dimensions, 210.0);
    box_1.print_props();
}
