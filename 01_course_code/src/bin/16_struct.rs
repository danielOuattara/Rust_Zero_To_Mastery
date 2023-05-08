struct ShippingBox {
    depth: i32,
    width: i32,
    height: i32,
}

//------------------------------
struct GroceryItem {
    stock: i32,
    price: f64, // double
}

//------------------------------
fn main() {
    let my_box_1 = ShippingBox {
        depth: 3,
        width: 2,
        height: 5,
    };

    let height = my_box_1.height;
    println!("The box is {:?} units tall", height);

    //--------------------------------------------

    let cereal = GroceryItem {
        stock: 10,
        price: 2.99,
    };

    println!("stock in cereal: {:?}", cereal.stock);
    println!("price in cereal: {:?}", cereal.price);
}
