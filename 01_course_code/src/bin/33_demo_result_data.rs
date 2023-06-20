#[derive(Debug)]

enum MenuChoice {
    MainMenu,
    Start,
    Quit,
}

fn get_choice(input: &str) -> Result<MenuChoice, String> {
    match input {
        "mainmenu" => Ok(MenuChoice::MainMenu),
        "start" => Ok(MenuChoice::Start),
        "quit" => Ok(MenuChoice::Quit),
        _ => Err("unable to find menu".to_owned()),
    }
}

fn main() {
    let choice = get_choice("mainmenu");
    println!("choice = {:?}", choice);
}

// stopped @ 5H19
