struct ShippingBox {
    depth: u8,
    width: u8,
    height: u8,
}

fn shipping_box() {
    let my_box_1: ShippingBox = ShippingBox {
        depth: 3,
        width: 2,
        height: 5,
    };

    let height = my_box_1.height;
    println!("The box is {:?} units tall", height);
    println!("The box is {:?} units tall", my_box_1.height);
    println!("The box is {:?} units height", my_box_1.height);
    println!("----------------------------------")
}

//-----------------------------------------------------------------
// Demo
struct GroceryItem {
    stock: u16,
    price: f32, // double
}

fn grocery_stock() {
    let cereal: GroceryItem = GroceryItem {
        stock: 10,
        price: 2.99,
    };

    println!("stock in cereal: {:?}", cereal.stock);
    println!("price in cereal: {:?}", cereal.price);
    println!("----------------------------------")
}

//-----------------------------------------------------------------
fn main() {
    shipping_box();
    grocery_stock();
}
