/*

 Option data
=============

° A type that may be one of two things
    - some data of a specified type
    - Nothing

° Used in scenarios where data may not be required OR is unavailable
    - Unable to find something
    - Ran out of items in a list
    - Form filed not filled out
 */

fn main() {
    /* Definition + Example 1
    ------------------------------*/
    enum Option<T> {
        Some(T),
        None,
    }

    struct Customer {
        email: String,
        age: Option<i32>,
    }
    let daniel = Customer {
        email: "daniel@email.com".to_owned(),
        age: Option::Some(39),
    };

    let julie = Customer {
        email: "julie@email.com".to_owned(),
        age: Option::None,
    };

    match daniel.age {
        Option::Some(age) => println!("customer is {:?} years old", age),
        Option::None => println!("customer age NOT provided"),
    }
    match julie.age {
        Option::Some(age) => println!("customer is {:?} years old", age),
        Option::None => println!("customer age NOT provided"),
    }

    let grocery_item_name = find_quantity(&"bananas");

    println!("{:?}", grocery_item_name)
}

/* Example 2
-------------- */

struct GroceryItem {
    name: String,
    quantity: i32,
}

fn find_quantity(name: &str) -> Option<i32> {
    let groceries = vec![
        GroceryItem {
            name: "bananas".to_owned(),
            quantity: 4,
        },
        GroceryItem {
            name: "eggs".to_owned(),
            quantity: 12,
        },
        GroceryItem {
            name: "bread".to_owned(),
            quantity: 2,
        },
    ];

    for item in groceries {
        if item.name == name {
            return Some(item.quantity);
        }
    }
    None
}

/*  STOPPEd @ 4h46' */
