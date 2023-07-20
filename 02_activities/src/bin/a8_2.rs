// Topic: Organizing similar data using structs
//
// Requirements:
// * Print the flavor of a drink and it's fluid ounces
//
// Notes:
// * Use an enum to create different flavors of drinks
// * Use a struct to store drink flavor and fluid ounce information
// * Use a function to print out the drink flavor and ounces
// * Use a match expression to print the drink flavor

//-------------------------------------------------------------------------

enum Flavor {
    Sparkling,
    Sweet,
    Alcohol,
}

struct Drink {
    flavor: Flavor,
    fluid_oz: f64,
}

fn print_drink_info_2(drink: Drink) {
    match drink.flavor {
        Flavor::Sparkling => println!("flavor: sparkling"),
        Flavor::Sweet => println!("flavor: sweet"),
        Flavor::Alcohol => println!("flavor: alcohol"),
    };

    println!("oz: {:?}", drink.fluid_oz);
}

//--------------------------------------------------------------------------

fn main() {
    let drink = Drink {
        flavor: Flavor::Sparkling,
        fluid_oz: 8.0,
    };
    print_drink_info_2(drink);
    //------
    let fruity = Drink {
        flavor: Flavor::Sweet,
        fluid_oz: 12.5,
    };
    print_drink_info_2(fruity);
}
