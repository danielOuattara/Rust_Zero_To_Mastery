struct ShippingBox {
    depth: u8,
    width: u8,
    height: u8,
}

//------------------------------
struct GroceryItem {
    stock: u16,
    price: f32, // double
}

//------------------------------
fn main() {
    let my_box_1: ShippingBox = ShippingBox {
        depth: 3,
        width: 2,
        height: 5,
    };

    let height: u8 = my_box_1.height;
    println!("The box is {:?} units tall", height);
    println!("The box is {:?} units tall", my_box_1.height);

    //--------------------------------------------

    let cereal: GroceryItem = GroceryItem {
        stock: 10,
        price: 2.99,
    };

    println!("stock in cereal: {:?}", cereal.stock);
    println!("price in cereal: {:?}", cereal.price);
}
