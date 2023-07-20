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

//--------------------------------------------------
enum Drinks {
    Water,
    Beer,
    Coffee,
}

struct DrinksIngredient {
    flavor: String,
    volume: i32,
}

fn print_drink_info(drink: Drinks) {
    match drink {
        Drinks::Water => {
            let water = DrinksIngredient {
                flavor: "water".to_string(),
                volume: 36,
            };
            println!(
                "The flavor is {} and ounce is {}",
                water.flavor, water.volume
            )
        }

        Drinks::Beer => {
            let beer = DrinksIngredient {
                flavor: "houblon, maize".to_string(),
                volume: 12,
            };
            println!("The flavor is {} and ounce is {}", beer.flavor, beer.volume)
        }

        Drinks::Coffee => {
            let coffee = DrinksIngredient {
                flavor: "coffee grains & water".to_string(),
                volume: 3,
            };
            println!(
                "The flavor is {} and ounce is {}",
                coffee.flavor, coffee.volume
            )
        }
    }
}

fn main() {
    //---------------------------------
    print_drink_info(Drinks::Water);
}
