// Topic: Advanced match
//
// Requirements:
// * Print out a list of tickets and their information for an event
// * Tickets can be Backstage, Vip, and Standard
// * Backstage and Vip tickets include the ticket holder's name
// * All tickets include the price
//
// Notes:
// * Use an enum for the tickets with data associated with each variant
// * Create one of each ticket and place into a vector
// * Use a match expression while iterating the vector to print the ticket info

enum Tickets {
    Backstage(String, i32),
    Vip(String, i32),
    Standard(i32),
}

fn main() {
    let tickets = vec![
        Tickets::Backstage("Gaia".to_owned(), 83),
        Tickets::Vip("Daniel".to_owned(), 103),
        Tickets::Standard(23),
    ];

    for ticket in &tickets {
        match ticket {
            Tickets::Backstage(name, price) => {
                println!(
                    "Backstage ticket: price is {:?} € name is {:?}",
                    price, name
                )
            }
            Tickets::Vip(name, price) => {
                println!("Vip ticket: price is {:?} € name is {:?}", price, name)
            }
            Tickets::Standard(price) => println!("Backstage ticket: price is {:?} €", price),
        }
    }
}
