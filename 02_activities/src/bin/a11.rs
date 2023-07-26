// Topic: Ownership
//
// Requirements:
// * Print out the quantity and id number of a grocery item
//
// Notes:
// * Use a struct for the grocery item
// * Use two i32 fields for the quantity and id number
// * Create a function to display the quantity, with the struct as a parameter
// * Create a function to display the id number, with the struct as a parameter

struct GroceryItem {
    id: i32,
    quantity: i32,
}

fn display_item_id(item: &GroceryItem) {
    println!("item id = {:?}", item.id);
}

fn display_item_quantity(item: &GroceryItem) {
    println!("item quantity = {:?}", item.quantity);
}

fn main() {
    let soda = GroceryItem {
        id: 12654,
        quantity: 69,
    };
    display_item_id(&soda);
    display_item_quantity(&soda);
}
