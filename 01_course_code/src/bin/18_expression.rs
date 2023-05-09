/*
° Rust is an expression-based language
    - most things are evaluated and return some value
° Expression value coalesce to a single point
    - can be used for nesting logic
*/

enum Menu {
    Burger,
    Fries,
    Drink,
}

enum Access {
    Admin,
    Manager,
    User,
    Guest,
}

fn main() {
    // Example 1
    let _my_num = 3;

    let _is_let_5 = if _my_num < 5 { true } else { false };
    println!("{_is_let_5}");

    let _is_let_5 = _my_num > 5; // shortcut
    println!("{_is_let_5}");

    // Example 2
    let _my_num = 3;
    let _message = match _my_num {
        1 => "hello",
        _ => "goodbye",
    };
    println!("{_message}");

    // Example 3
    let paid = true;
    let item = Menu::Drink;
    let drink_type = "Water";
    let order_placed = match item {
        Menu::Drink => {
            if drink_type == "Water" {
                true
            } else {
                false
            }
        }
        _ => true,
    };

    // secret file; admin only

    let access_level = Access::Guest;
    let can_access_file = match access_level {
        Access::Admin => true,
        _ => false,
    };

    println!("Accessibility is {:?}", can_access_file);
}
