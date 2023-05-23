enum Mouse {
    LeftClick,
    RightClick,
    MiddleClick,
    Scroll(i32),
    Move(i32, i32),
}

enum PromoDiscount {
    NewUser,
    Holiday(String),
}

enum Discount {
    Percent(f64),
    Flat(i32),
    Promo(PromoDiscount),
    Custom(String),
}

fn main() {
    where_to(Direction::Up);
}

/*  STOPPED  4H20'  */
