enum Access {
    Full,
}

fn one_two_three() -> (i32, i32, i32) {
    return (1, 2, 3);
}

fn main() {
    // create first tuple
    let first_tuple = one_two_three();
    println!("{:?}", first_tuple); // (1, 2, 3)

    // destructuring
    let (x, y, z) = one_two_three();
    println!("{:?} , {:?}", x, first_tuple.0); // 1
    println!("{:?} , {:?}", y, first_tuple.1); // 2
    println!("{:?} , {:?}", z, first_tuple.2); // 3

    //-----------------------------

    let (employee, _access) = ("Jake", Access::Full);
    println!("employee = {:?},", employee);
    // println!("access = {},", access);
    // println!("{}", Access::Full);

    //-----------------------------

    let coords = (2, 3, 4);
    println!("{:?}, {:?}, {:?}", coords.0, coords.1, coords.2);

    let (x, y, z) = coords;
    println!("{:?}, {:?}, {:?} ", x, y, z);

    let _user_info = ("Emma", 20);
    // better is :
    let (name, age) = ("Emma", 20);
    println!("{:?}, {:?}", name, age);

    // for large number of elements in a tuple, prefer use a struct

    // bad example
    let _favorites = ("red", 14, "Texas", "Pizza", "Super Tv Show, home");

    struct FavoritesItems {
        favorite_color: String,
        favorite_number: i32,
        favorite_state: String,
        favorite_food: String,
        favorite_tv_show: String,
        favorite_place: String,
    }

    let _my_favorite = FavoritesItems {
        favorite_color: "Red".to_string(),
        favorite_number: 32,
        favorite_state: "Texas".to_string(),
        favorite_food: "Pizza".to_string(),
        favorite_tv_show: "Song & Dance".to_string(),
        favorite_place: "Home".to_string(),
    };
}
