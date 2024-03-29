/*

=== OWNERSHIP ===

MANAGING MEMORY
------------------
° Programs must track memory
    - if they fail to do so, a memory "leak" occurs
    - a memory leak is when a program fails to track which memory
      is being used and has to reserve a new piece of memory

° Rust utilizes an "ownership" model to manage memory
    - the "owner" of the memory is responsible for cleaning up the memory

° Memory can either be "moved" or "borrowed"

*/

/*  new Demo on ownership
--------------------------- */

struct Book {
    pages: i32,
    rating: i32,
}

fn display_page_count(book: &Book) {
    println!("pages = {:?}", book.pages);
}

fn display_rating(book: &Book) {
    println!("rating = {:?}", book.rating);
}

fn main() {
    let book = Book {
        pages: 355,
        rating: 9,
    };

    display_page_count(&book);
    display_rating(&book);
}
