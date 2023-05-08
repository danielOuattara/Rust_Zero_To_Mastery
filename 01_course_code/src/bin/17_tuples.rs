enum Access {
    Full,
}

fn one_two_three() -> (i32, i32, i32) {
    return (1, 2, 3);
}

fn main() {
    let first_tuple = one_two_three();
    println!("{:?}", first_tuple); // (1, 2, 3)

    let (x, y, z) = one_two_three(); // destructuring
    println!("{:?}", x);
    println!("{:?}", y);
    println!("{:?}", z);

    let (employee, access) = ("Jake", Access::Full);
    println!("employee = {:?},", employee);

    //-----------------------------
    let coords = (2, 3, 4);
    println!("{:?}, {:?}, {:?}", coords.0, coords.1, coords.2);

    let (x, y, z) = coords;
    println!("{:?}, {:?}, {:?} ", x, y, z);

    let user_info = ("Emma", 20);
    let (name, age) = user_info;
    println!("{:?}, {:?}", name, age);

    // for large number of elements in a tuple, prefer use a struct

    let favorites = ("red", 14, "Texas", "Pizza", "Super Tv Show, home");

    struct FavoritesItems {
        favorite_color: String,
        favorite_number: i32,
        favorite_state: String,
        favorite_food: String,
        favorite_tv_show: String,
        favorite_place: String,
    }
    let my_favorite = FavoritesItems {
        favorite_color: "Red".to_string(),
        favorite_number: 32,
        favorite_state: "Texas".to_string(),
        favorite_food: "Pizza".to_string(),
        favorite_tv_show: "Song & Dance".to_string(),
        favorite_place: "Home".to_string(),
    };
}
